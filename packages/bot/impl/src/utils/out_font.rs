use std::{collections::HashMap, sync::LazyLock};
use fontdue::Font;
use tiny_skia::{Path, PathBuilder};
use ttf_parser::{Face, GlyphId, Rect};
use crate::resources::ROBOTO_TTF;

#[allow(unused)]
pub struct OutlinedFont {
    pub font: Font,
    pub glyphs: HashMap<GlyphId, (Glyph, Rect)>,
    pub units_per_em: f32,
    pub descender: f32,
    pub height: f32,
}

thread_local! {
    static ROBOTO: LazyLock<OutlinedFont> = LazyLock::new(|| { 
        OutlinedFont::from_bytes(ROBOTO_TTF).unwrap()
    });
}

impl OutlinedFont {
    pub fn from_bytes(
        font_data: &[u8]
    ) -> Result<Self, String> {
        let font = Font::from_bytes(
            font_data, 
            fontdue::FontSettings::default()
        )?;

        let face = ttf_parser::Face::parse(font_data, 0)
            .map_err(|e| e.to_string())?;
        let units_per_em = face.units_per_em() as f32;
        let descender = face.descender() as f32;
        let height = face.height() as f32;
        let glyphs = Self::build_glyphs(&face)?;

        Ok(Self {
            font,
            glyphs,
            units_per_em,
            descender,
            height,
        })
    }

    fn build_glyphs(
        face: &Face
    ) -> Result<HashMap<GlyphId, (Glyph, Rect)>, String> {

        let mut glyphs = HashMap::new();

        for id in 0..face.number_of_glyphs() {
            let mut path_builder = PathBuilder::new();
            let mut builder = GlyphBuilder::new(&mut path_builder);
            let gid = ttf_parser::GlyphId(id);
            if let Some(bbox) = face.outline_glyph(gid, &mut builder) {
                let glyph = Glyph::new(path_builder.finish().unwrap());
                glyphs.insert(gid, (glyph, bbox));
            };
        }

        Ok(glyphs)
    }

    pub fn roboto<F, R>(
        f: F
    ) -> R where 
        F: FnOnce(&OutlinedFont) -> R {
        ROBOTO.with(|b| f(b))
    }

    pub fn calc_text_width(
        &self,
        text: &str,
        size: f32
    ) -> f32 {
        let mut w = 0.0;
        for char in text.chars() {
            let met = self.font.metrics(char, size);
            w += met.advance_width;
        }
        w
    }
}

pub struct Glyph {
    pub path: Path,
}

impl Glyph {
    pub fn new(
        path: Path
    ) -> Self {
        Self {
            path,
        }
    }
}

struct GlyphBuilder<'a> {
    builder: &'a mut PathBuilder,
}

impl<'a> GlyphBuilder<'a> {
    pub fn new(
        builder: &'a mut PathBuilder
    ) -> Self {
        Self {
            builder,
        }
    }
}

impl ttf_parser::OutlineBuilder for GlyphBuilder<'_> {
    fn move_to(
        &mut self, 
        x: f32, 
        y: f32
    ) {
        self.builder.move_to(x, -y);
    }

    fn line_to(
        &mut self, 
        x: f32, 
        y: f32
    ) {
        self.builder.line_to(x, -y);
    }

    fn quad_to(
        &mut self, 
        x1: f32, 
        y1: f32, 
        x: f32, 
        y: f32
    ) {
        self.builder.quad_to(x1, -y1, x, -y);
    }

    fn curve_to(
        &mut self, 
        x1: f32, 
        y1: f32, 
        x2: f32, 
        y2: f32, 
        x: f32, 
        y: f32
    ) {
        self.builder.cubic_to(x1, -y1, x2, -y2, x, -y);
    }

    fn close(
        &mut self
    ) {
        self.builder.close();
    }
}


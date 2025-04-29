use core::f32;
use std::u32;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, 
    Layout, LayoutSettings, 
    TextStyle, VerticalAlign, WrapStyle
};
use image::{GenericImage, RgbaImage};
use tiny_skia::{
    Color, FillRule, LineCap, LineJoin, 
    Paint, PathBuilder, PixmapMut, Stroke, 
    Transform
};
use ttf_parser::GlyphId;
use super::out_font::OutlinedFont;

pub struct TextOutline {
    pub size: f32,
    pub color: Color,
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(
        x: u32,
        y: u32
    ) -> Self {
        Self {
            x,
            y
        }
    }
}

pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}

impl From<&Point> for PointF32 {
    fn from(
        value: &Point
    ) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

pub struct Canvas<'a> {
    image: &'a mut RgbaImage,
    width: u32,
    height: u32
}

impl<'a> Canvas<'a> {
    pub fn new(
        image: &'a mut RgbaImage
    ) -> Self {
        let width = image.width();
        let height = image.height();
        Self {
            image,
            width,
            height,
        }
    }

    pub fn blit_image_at(
        &mut self,
        src: &RgbaImage,
        x: u32,
        y: u32
    ) -> Result<(), String> {
        self.image.copy_from(src, x, y)
            .map_err(|e| e.to_string())
    }

    pub fn draw_rect(
        &mut self,
        width: u32,
        height: u32,
        color: &Color,
        at: &Point
    ) {
        const STROKE_W: f32 = 3.0;
        
        let mut canvas_buf = self.image.as_flat_samples_mut();
        let mut pixmap = PixmapMut::from_bytes(
            bytemuck::cast_slice_mut(canvas_buf.as_mut_slice()), self.width, self.height
        ).unwrap();

        let mut paint = Paint::default();
        paint.set_color(color.clone());
        paint.anti_alias = true;

        let path = {
            let at: PointF32 = at.into();
            let mut pb = PathBuilder::new();
            pb.move_to(at.x, at.y);
            pb.line_to(at.x + width as f32, at.y);
            pb.line_to(at.x + width as f32, at.y + height as f32);
            pb.line_to(at.x, at.y + height as f32);
            pb.close();
            pb.finish().unwrap()
        };

        let mut stroke = Stroke::default();
        stroke.width = STROKE_W;
        stroke.line_cap = LineCap::Round;

        pixmap.stroke_path(
            &path, 
            &paint, 
            &stroke, 
            Transform::identity(), 
            None
        );
    }

    pub fn draw_text_ex(
        &mut self,
        text: &str,
        size: f32,
        font: &OutlinedFont,
        color: &Color,
        x: u32,
        y: u32,
        max_width: Option<f32>,
        max_height: Option<f32>,
        outline: Option<&TextOutline>
    ) {
        let mut canvas_buf = self.image.as_flat_samples_mut();
        let mut pixmap = PixmapMut::from_bytes(
            bytemuck::cast_slice_mut(canvas_buf.as_mut_slice()), self.width, self.height
        ).unwrap();

        let mut paint = Paint::default();
        paint.set_color(color.clone());
        paint.anti_alias = true;
        
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings{
            x: 0.0,
            y: 0.0,
            max_width,
            max_height,
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
            line_height: 1.0,
            wrap_style: WrapStyle::Word,
            wrap_hard_breaks: true,
        });
    
        layout.append(&[font.font.clone()], &TextStyle::new(text, size, 0));
        
        let scale = size / font.units_per_em;
        let height = font.height * scale;

        let mut stroke = Stroke::default();
        stroke.line_join = LineJoin::Round;
        stroke.line_cap = LineCap::Round;

        let mut stroke_paint = Paint::default();
        stroke_paint.anti_alias = true;
        if let Some(outline) = outline {
            stroke.width = outline.size / scale;
            stroke_paint.set_color(
                outline.color
            );
        }

        let x = x as f32;
        let y = y as f32;

        if outline.is_some() {
            let mut row_y = f32::MIN;
            for (i, char) in text.chars().enumerate() {
                let gid = font.font.lookup_glyph_index(char);
                if let Some((glyph, _bbox)) = font.glyphs.get(&GlyphId(gid)) {
                    let gpos = &layout.glyphs()[i];
                    if (row_y - gpos.y).abs() > height {
                        row_y = gpos.y;
                    }
                    let transf = Transform::from_scale(scale, scale)
                        .post_translate(
                            x + gpos.x, 
                            y + row_y + height / 2.0
                        );

                    pixmap.stroke_path(
                        &glyph.path, &stroke_paint, &stroke, transf, None
                    );
                }
            }
        }

        let mut row_y = f32::MIN;
        for (i, char) in text.chars().enumerate() {
            let gid = font.font.lookup_glyph_index(char);
            if let Some((glyph, _bbox)) = font.glyphs.get(&GlyphId(gid)) {
                let gpos = &layout.glyphs()[i];
                if (row_y - gpos.y).abs() > height {
                    row_y = gpos.y;
                }
                let transf = Transform::from_scale(scale, scale)
                    .post_translate(
                        x + gpos.x, 
                        y + row_y + height / 2.0
                    );

                pixmap.fill_path(
                    &glyph.path, &paint, FillRule::Winding, transf, None
                );
            }
        }
    }
    
    pub fn draw_text(
        &mut self,
        text: &str,
        size: f32,
        font: &OutlinedFont,
        color: &Color,
        x: u32,
        y: u32,
        max_width: Option<f32>,
        max_height: Option<f32>,
    ) {
        self.draw_text_ex(
            text, size, font, color, x, y, 
            max_width,
            max_height,
            Some(&TextOutline{
                size: 4.0,
                color: Color::BLACK,
            })
        )
    }
}

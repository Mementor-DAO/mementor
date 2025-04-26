use std::collections::HashMap;
use ic_stable_structures::Storable;
use image::{Rgba, RgbaImage};
use sha2::{
    digest::generic_array::GenericArray, 
    Digest, Sha256
};
use tantivy::schema::{
    IndexRecordOption, OwnedValue, 
    TextFieldIndexing, TextOptions, 
    STORED 
};
use tiny_skia::Color;
use ic_llm::Model;
use crate::{
    storage::{
        image::ImageStorage, 
        thumb::ThumbStorage
    }, 
    types::{
        meme::{MemeId, MEME_ID_SIZE}, 
        meme_tpl::{MemeTpl, MemeTplTextBox}, 
        thumb::{THUMB_HEIGHT, THUMB_WIDTH}
    }, 
    utils::{
        canvas::{Canvas, Point, TextOutline}, 
        full_text_indexer::{
            Field, FieldOptions, FullTextIndexer
        }, 
        out_font::OutlinedFont, 
        rng::gen_range 
    }
};

pub const MAX_MEMES: usize = 4;
const FONT_SIZE: f32 = 32.0; //px
const PADDING: usize = 8;
const CAPTION_CREATE_PROMPT: &str = "You're a meme expert. Given a meme with this image description: \"{description}\" and this usage suggestion: \"{usage}\", create {num_boxes} short captions, each with no more than 3 words, that together tell a story. Be funny and creative! Return only the captions as a JSON array of strings";

pub type MemeTplId = u32;

#[derive(Clone)]
pub struct MemeService {
    memes: HashMap<MemeTplId, MemeTpl>,
    finder: FullTextIndexer,
    hasher: Sha256,
}

impl MemeService {
    pub fn new(
        memes_json_bytes: Vec<u8>,
        index_tar_bytes: Vec<u8>
    ) -> Self {

        let memes = serde_json::from_slice::<Vec<MemeTpl>>(
            memes_json_bytes.as_slice()
        ).unwrap()
            .iter()
            .map(|m| (m.id, m.clone()))
            .collect::<HashMap<MemeTplId, MemeTpl>>();
        
        let finder = FullTextIndexer::from_tar(
            index_tar_bytes.to_vec().into_boxed_slice(), 
            "index/".to_string(),
            &Self::get_index_fields(),
            "id"
        ).unwrap();

        Self {
            memes,
            finder,
            hasher: Sha256::new(),
        }
    }

    fn get_index_fields(
    ) -> Vec<Field> {
        let text_en_stem = TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("en_stem")
                    .set_index_option(IndexRecordOption::WithFreqs)
            );

        vec![
            Field { name: "id".to_string(), opts: FieldOptions::Numeric(STORED.into()) },
            Field { name: "name".to_string(), opts: FieldOptions::Text(text_en_stem.clone()) },
            Field { name: "description".to_string(), opts: FieldOptions::Text(text_en_stem.clone()) },
            Field { name: "usage".to_string(), opts: FieldOptions::Text(text_en_stem.clone()) },
            Field { name: "keywords".to_string(), opts: FieldOptions::Text(text_en_stem) },
        ]
    }

    pub fn load(
        &self,
        id: &MemeTplId
    ) -> Option<&MemeTpl> {
        self.memes.get(id)
    }

    pub fn search(
        &self,
        what: &str
    ) -> Vec<MemeTpl> {
        let Ok(ids) = self.finder.search(
            what, 
            &"id", 
            MAX_MEMES * 5,
            |v: &OwnedValue| {
                match v {
                    OwnedValue::U64(s) => *s as u32,
                    _ => 0
                }
            })
        else {
            return vec![];
        };

        let mut memes = ids.iter()
            .map(|id| {
                self.memes.get(id)
                    .cloned()
                    .unwrap_or_else(|| MemeTpl::default())
            })
            .filter(|m| m.id != 0)
            .collect::<Vec<_>>();

        if memes.len() > MAX_MEMES {
            let mut out = vec![];
            while out.len() < MAX_MEMES {
                let index = gen_range(0..memes.len());
                out.push(memes.swap_remove(index));
            }
            out
        }
        else {
            memes
        }
    }

    pub fn gen_preview(
        memes: &Vec<MemeTpl>,
        font: &OutlinedFont
    ) -> Result<RgbaImage, String> {
        
        let mut out = RgbaImage::from_pixel(
            ((PADDING + THUMB_WIDTH) * memes.len() + PADDING) as _, 
            (PADDING*2 + THUMB_HEIGHT) as _,
            Rgba([239, 239, 239, 255])
        );

        let mut canvas = Canvas::new(&mut out);

        let text_color = Color::from_rgba8(0xf7, 0x78, 0x00, 0xff);

        let mut x = PADDING;
        for meme in memes {
            let num_text = meme.id.to_string();
            if let Some(thumb) = ThumbStorage::load(&meme.id) {
                canvas.blit_image_at(&thumb, x as _, PADDING as _)
                    .map_err(|e| e.to_string())?;

                canvas.draw_text(
                    &num_text, 
                    36.0, font, &text_color, 
                    x as _, PADDING as _,
                    Some(thumb.width() as f32), Some(thumb.height() as f32)
                );
    
                canvas.draw_rect(
                    THUMB_WIDTH as _, THUMB_HEIGHT as _,
                    &text_color, 
                    &Point::new(x as _, PADDING as _)
                );
            }

            x += THUMB_WIDTH + PADDING;
        }

        Ok(out)
    }

    pub fn gen_image(
        meme: &MemeTpl,
        texts: &Vec<String>,
        font: &OutlinedFont
    ) -> Result<RgbaImage, String> {
        
        if let Some(mut img) = ImageStorage::load(&meme.id).clone() {
            Self::draw_texts(texts, meme, font, FONT_SIZE, &mut img);
            Ok(img)
        }
        else {
            Err("Not found".to_string())
        }
    }

    pub fn draw_texts(
        texts: &Vec<String>,
        meme: &MemeTpl,
        font: &OutlinedFont,
        size: f32,
        dest: &mut RgbaImage
    ) {
        let width = dest.width() as f32;
        let height = dest.height() as f32;
        
        let hscale = width / meme.width as f32;
        let vscale = height / meme.height as f32;
        
        let boxes = if meme.boxes.len() > 0 {
            meme.boxes.clone()
        }
        else {
            vec![
                MemeTplTextBox{ 
                    left: width / hscale * 0.1, 
                    top: height / vscale * 0.1, 
                    width: width / hscale * 0.8, 
                    height: size / vscale * 1.5, 
                    rotation: None
                },
                MemeTplTextBox{ 
                    left: width / hscale * 0.2, 
                    top: height / vscale * 0.8, 
                    width: width / hscale * 0.6, 
                    height: size / vscale, 
                    rotation: None
                }
            ]
        };

        let color = Color::BLACK;

        let outline = TextOutline {
            size: 4.0,
            color: Color::WHITE,
        };

        let mut canvas = Canvas::new(dest);

        for (t, bx) in boxes.iter().enumerate() {
            if t < texts.len() {
                let text = &texts[t];
                
                let x = bx.left * hscale;
                let y = bx.top * vscale;
                let w = bx.width * hscale;
                let h = bx.height * vscale;

                let text_width = font.calc_text_width(text, size) * 1.2;
                let font_size = if text_width > w {
                    (size * (w / text_width)).max(size / 2.0)
                }
                else {
                    size
                };
                
                canvas.draw_text_ex(
                    text, 
                    font_size, font, &color, 
                    x as _, y as _, Some(w), Some(h),
                    Some(&outline)
                );
            }
        }
    }

    pub fn calc_id(
        &mut self,
        tpl: &MemeTpl, 
        texts: &Vec<String>
    ) -> MemeId {
        // 1st: hash any the relevant data
        let mut arr: Vec<Vec<u8>> = vec![
            tpl.id.to_bytes().to_vec(),
        ];
        
        texts.iter().for_each(|t| 
            arr.push(t.trim().to_uppercase().to_bytes().to_vec())
        );

        self.hasher.update(
            arr.iter().flatten().cloned().collect::<Vec<_>>()
        );
        
        let mut num256 = GenericArray::from([0u8; 32]);
        self.hasher.finalize_into_reset(&mut num256);

        // 2nd: truncate to MEME_ID_SIZE
        let mut id = hex::encode_upper(&num256.as_slice());
        id.truncate(MEME_ID_SIZE);
        id
    }
    
    pub async fn gen_captions(
        tpl: &MemeTpl
    ) -> Result<Vec<String>, String> {
        let num_captions = if tpl.boxes.len() == 0 {
            2
        } 
        else {
            tpl.boxes.len()
        };
        
        let prompt = CAPTION_CREATE_PROMPT
            .replace("{description}", &tpl.description.replace('"', "'"))
            .replace("{usage}", &tpl.usage.replace('"', "'"))
            .replace("{num_boxes}", &num_captions.to_string());
        
        let res = ic_llm::prompt(Model::Llama3_1_8B, prompt)
            .await;

        let captions: Vec<String> = serde_json::from_str(&res.trim())
            .map_err(|err| err.to_string())?;

        Ok(captions)
    }
}


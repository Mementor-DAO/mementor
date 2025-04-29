use std::{cell::RefCell, io::Cursor};
use ic_stable_structures::BTreeMap;
use image::RgbaImage;
use crate::{
    memory::{get_thumbs_memory, Memory}, 
    services::meme::{MemeTplId, MemeService},
    types::{
        meme_tpl::MemeTpl, 
        thumb::{
            Thumb, THUMB_FONT_SIZE, THUMB_FORMAT, THUMB_HEIGHT, THUMB_WIDTH
        }
    }, utils::{out_font::OutlinedFont, image::rgba8_to_rgb8}
};

thread_local! {
    static THUMBS: RefCell<BTreeMap<MemeTplId, Thumb, Memory>> = RefCell::new(
        BTreeMap::init(
            get_thumbs_memory()
        )
    );
}

pub struct ThumbStorage;

impl ThumbStorage {
    pub fn save(
        id: MemeTplId,
        data: Vec<u8>
    ) {
        THUMBS.with_borrow_mut(|thumbs| {
            thumbs.insert(
                id, 
                Thumb {
                    data,
                }
            )
        });
    }

    fn draw_texts(
        meme: &MemeTpl,
        thumb: &mut RgbaImage,
        font: &OutlinedFont
    ) {
        let texts = (0..meme.boxes.len().max(2))
            .map(|num| format!("TEXT {}", num+1))
            .collect();
        
        MemeService::draw_texts(&texts, meme, font, THUMB_FONT_SIZE, thumb);
    }

    pub fn load(
        id: &MemeTplId
    ) -> Option<RgbaImage> {
        let Some(img) = THUMBS.with_borrow(|thumbs| {
            thumbs.get(id)
        }) else {
            return None;
        };

        let Ok(thumb) = image::load_from_memory_with_format(
            &img.data, THUMB_FORMAT
        ) else {
            return None;
        };

        Some(thumb.to_rgba8())
    }

    pub fn gen_thumb(
        img: &RgbaImage,
        meme: &MemeTpl,
        font: &OutlinedFont
    ) -> Result<Vec<u8>, String> {
        let mut thumb = image::imageops::thumbnail(
            img, 
            THUMB_WIDTH as _, 
            THUMB_HEIGHT as _
        );

        Self::draw_texts(&meme, &mut thumb, font);

        let rgb_img = rgba8_to_rgb8(&thumb);

        let mut jpg: Vec<u8> = Vec::new();
        rgb_img.write_to(&mut Cursor::new(&mut jpg), THUMB_FORMAT)
            .map_err(|e| e.to_string())?;

        Ok(jpg)
    }
}
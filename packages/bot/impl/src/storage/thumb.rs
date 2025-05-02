use std::cell::RefCell;
use ic_stable_structures::BTreeMap;
use image::RgbaImage;
use crate::{
    memory::{get_thumbs_memory, Memory}, 
    services::meme::MemeTplId,
    types::
        thumb::{
            Thumb, THUMB_FORMAT
        }
    
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
}
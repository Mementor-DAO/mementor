use std::cell::RefCell;
use ic_stable_structures::BTreeMap;
use image::RgbaImage;
use crate::{
    memory::{get_images_memory, Memory}, 
    services::meme::MemeTplId, 
    types::image::{
        Image, IMAGE_FORMAT
    }
};

thread_local! {
    static IMAGES: RefCell<BTreeMap<MemeTplId, Image, Memory>> = RefCell::new(
        BTreeMap::init(
            get_images_memory()
        )
    );
}

pub struct ImageStorage;

impl ImageStorage {
    pub fn save(
        id: MemeTplId,
        data: Vec<u8>
    ) {
        IMAGES.with_borrow_mut(|images| {
            images.insert(
                id, 
                Image {
                    data,
                }
            );
        });
    }

    pub fn load(
        id: &MemeTplId
    ) -> Option<RgbaImage> {
        let Some(img) = IMAGES.with_borrow(|images| {
            images.get(id)
        }) else {
            return None;
        };

        let Ok(img) = image::load_from_memory_with_format(
            &img.data, IMAGE_FORMAT,
        ) else {
            return None;
        };

        Some(img.to_rgba8())
    }
}
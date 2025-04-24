use std::{cell::RefCell, io::Cursor};
use ic_stable_structures::BTreeMap;
use image::{imageops, RgbaImage};
use crate::{
    memory::{get_images_memory, Memory}, 
    services::meme::MemeTplId, 
    types::image::{
        Image, IMAGE_FORMAT, IMAGE_HEIGHT, IMAGE_WIDTH
    }, utils::image::rgba8_to_rgb8
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

    pub fn resize(
        img: &RgbaImage,
        width: u32,
        height: u32
    ) -> RgbaImage {
        let w = img.width();
        let h = img.height();

        if w == width && h == height {
            img.clone()
        } else {
            if w >= h {
                let h = (height as f32 * (h as f32 / w as f32)) as u32;
                imageops::resize(img, width, h, imageops::FilterType::Nearest)
            }
            else {
                let w = (width as f32 * (w as f32 / h as f32)) as u32;
                imageops::resize(img, w, height, imageops::FilterType::Nearest)
            }
        }
    }

    pub fn process(
        img: &RgbaImage
    ) -> Result<Vec<u8>, String> {
        let rgb_img = rgba8_to_rgb8(&Self::resize(img, IMAGE_WIDTH, IMAGE_HEIGHT));

        let mut jpg: Vec<u8> = Vec::new();
        rgb_img.write_to(&mut Cursor::new(&mut jpg), IMAGE_FORMAT)
            .map_err(|e| e.to_string())?;

        Ok(jpg)
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
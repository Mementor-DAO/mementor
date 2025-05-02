use std::io::Cursor;
use image::RgbaImage;
use oc_bots_sdk_canister::env;
use bot_api::insert_image::{
    ImageInsertRequest, ImageInsertResponse
};
use crate::{
    services::meme, state::{self}, storage::{
        image::ImageStorage, 
        thumb::ThumbStorage
    }, 
    types::{
        image::{
            IMAGE_FORMAT, IMAGE_HEIGHT, 
            IMAGE_MAX_SIZE, IMAGE_WIDTH
        }, 
        thumb::THUMB_MAX_SIZE
    }, 
    utils::{
        image::{resize, rgba8_to_rgb8}, 
        out_font::OutlinedFont, 
        thumb::gen_thumb
    }
};

#[ic_cdk::update]
pub async fn insert_image(
    args: ImageInsertRequest
) -> ImageInsertResponse {
    if state::read(|s| s.administrator()) != env::caller() {
        return ImageInsertResponse::NotAuthorized;
    }

    match load_image(&args.data, &args.mime_type) {
        Some(img) => {
            let meme = meme::read(|s| {
                s.load(&args.id).cloned().unwrap()
            });
    
            match OutlinedFont::roboto(|font| gen_thumb(&img, &meme, font)) {
                Ok(buf) => {
                    if buf.len() as u32 > THUMB_MAX_SIZE {
                        return ImageInsertResponse::ThumbSizeTooBig;
                    }

                    ThumbStorage::save(
                        args.id.clone(),
                        buf
                    );
                },
                Err(_) => {
                    return ImageInsertResponse::ThumbGenerationFailed;
                }
            }

            let size = match process(&img) {
                Ok(buf) => {
                    if buf.len() as u32 > IMAGE_MAX_SIZE {
                        return ImageInsertResponse::ImageSizeTooBig;
                    }
        
                    let size = buf.len();
                    ImageStorage::save(args.id, buf);
                    size
                },
                Err(_) => {
                    return ImageInsertResponse::ImageGenerationFailed;
                }
            };

            ImageInsertResponse::Success(size)
        },
        None => {
            ImageInsertResponse::ImageLoadingFailed
        }
    }
}

fn load_image(
    data: &Vec<u8>,
    mime_type: &String
) -> Option<RgbaImage> {
    match image::load_from_memory_with_format(
        &data, 
        image::ImageFormat::from_mime_type(mime_type).unwrap()
    ) {
        Ok(img) => {
            Some(img.to_rgba8())
        },
        Err(_) => None,
    }
}

fn process(
    img: &RgbaImage
) -> Result<Vec<u8>, String> {
    let rgb_img = rgba8_to_rgb8(&resize(img, IMAGE_WIDTH, IMAGE_HEIGHT));

    let mut jpg: Vec<u8> = Vec::new();
    rgb_img.write_to(&mut Cursor::new(&mut jpg), IMAGE_FORMAT)
        .map_err(|e| e.to_string())?;

    Ok(jpg)
}


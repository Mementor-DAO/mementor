use image::RgbaImage;
use oc_bots_sdk_canister::env;
use crate::{
    api::insert_image::{
        ImageInsertRequest, ImageInsertResponse
    }, services::meme, state::{self}, storage::{
        image::ImageStorage, 
        thumb::ThumbStorage
    }, types::{
        image::IMAGE_MAX_SIZE, 
        thumb::THUMB_MAX_SIZE
    }, utils::out_font::OutlinedFont
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
                s.find_by_id(&args.id).cloned().unwrap()
            });
    
            match OutlinedFont::roboto(|font| ThumbStorage::gen_thumb(&img, &meme, font)) {
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

            let size = match ImageStorage::process(&img) {
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
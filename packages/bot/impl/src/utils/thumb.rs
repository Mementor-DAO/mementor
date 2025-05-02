use std::io::Cursor;
use image::RgbaImage;
use crate::{
    services::meme::MemeService, 
    types::{
        meme_tpl::MemeTpl, 
        thumb::{
            THUMB_FONT_SIZE, THUMB_FORMAT, 
            THUMB_HEIGHT, THUMB_WIDTH
        }
    }
};
use super::{
    image::rgba8_to_rgb8, out_font::OutlinedFont
};

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

    draw_texts(&meme, &mut thumb, font);

    let rgb_img = rgba8_to_rgb8(&thumb);

    let mut jpg: Vec<u8> = Vec::new();
    rgb_img.write_to(&mut Cursor::new(&mut jpg), THUMB_FORMAT)
        .map_err(|e| e.to_string())?;

    Ok(jpg)
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


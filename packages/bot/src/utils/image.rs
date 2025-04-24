use std::io::Cursor;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage, RgbaImage};
use oc_bots_sdk::types::ThumbnailData;

pub fn create_thumbnail(
    image: &[u8],
    w: u32,
    h: u32,
    format: ImageFormat
) -> Result<ThumbnailData, String> {
    let img = image::load_from_memory(image)
        .map_err(|e| e.to_string())?;
    let thumbnail = img.thumbnail(w, h);
    let mut bytes: Vec<u8> = Vec::new();
    thumbnail.write_to(&mut Cursor::new(&mut bytes), format)
        .map_err(|e| e.to_string())?;

    let mut data_uri = dataurl::DataUrl::new();
    data_uri.set_is_base64_encoded(true);
    data_uri.set_media_type(Some(format.to_mime_type().to_string()));
    data_uri.set_data(&bytes);

    Ok(ThumbnailData(data_uri.to_string()))
}

pub fn rgba8_to_rgb8(
    src: &RgbaImage
) -> RgbImage {
    let mut dst = ImageBuffer::new(src.width(), src.height());

    let mut src_iter = src.pixels();
    for dst_p in dst.pixels_mut() {
        let src_p = src_iter.next().unwrap();
        *dst_p = Rgb([
            src_p[0], src_p[1], src_p[2]
        ]);
    }

    dst
}
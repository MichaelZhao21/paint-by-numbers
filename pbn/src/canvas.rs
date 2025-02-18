use std::cmp;

use image::RgbImage;

pub fn shrink(img: RgbImage, max_size: u32) -> RgbImage {
    let (width, height) = img.dimensions();

    // Don't do anything if the image is already small enough
    if width <= max_size && height <= max_size {
        return img;
    }

    let scale = max_size as f32 / cmp::max(width, height) as f32;
    let new_size = (
        (width as f32 * scale) as u32,
        (height as f32 * scale) as u32,
    );

    return image::imageops::resize(&img, new_size.0, new_size.1, image::imageops::FilterType::Lanczos3);
}

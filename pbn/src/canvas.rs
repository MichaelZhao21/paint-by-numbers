use std::cmp;

use image::RgbImage;
use crate::kmeans;

/// Shrink an image to a maximum size while maintaining aspect ratio
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

/// Replace all pixels in an image with the nearest centroid
pub fn recolor(img: RgbImage, centroids: &Vec<image::Rgb<u8>>) -> RgbImage {
    let mut new_img = img.clone();
    for pixel in new_img.pixels_mut() {
        let mut min_distance = f64::MAX;
        let mut min_index = 0;
        for (i, centroid) in centroids.iter().enumerate() {
            let d = kmeans::distance(pixel, centroid);
            if d < min_distance {
                min_distance = d;
                min_index = i;
            }
        }
        *pixel = centroids[min_index];
    }
    new_img
}

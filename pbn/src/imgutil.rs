use image::{DynamicImage, ImageError, ImageFormat, ImageReader, ImageBuffer, Rgb};
use std::io::Cursor;

/// Converts a Vec<u8> to a DynamicImage.
pub fn vec_to_image(data: &Vec<u8>) -> Result<DynamicImage, ImageError> {
    let img = ImageReader::new(std::io::Cursor::new(data))
        .with_guessed_format()?  // Automatically detects format
        .decode()?;  // Decodes into a DynamicImage
    Ok(img)
}

/// Converts a DynamicImage to a Vec<u8> in PNG format.
pub fn image_to_vec(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, format: ImageFormat) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());

    // Encode the image into the buffer
    image.write_to(&mut buffer, format).expect("Failed to encode image");

    buffer.into_inner() // Return the Vec<u8>
}

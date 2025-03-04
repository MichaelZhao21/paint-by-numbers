mod canvas;
mod kmeans;
mod svg;
mod imgutil;

use wasm_bindgen::prelude::*;
use image::open;
use kmeans::kmeans;

#[wasm_bindgen]
pub fn test() -> String {
    console_error_panic_hook::set_once();
    println!("Hello from Rust!");
    "Hello from Rust!".to_string()
}

#[wasm_bindgen]
pub fn img_to_flat(input: Vec<u8>, k: i32, min_area: u32) -> Vec<u8> {
    console_error_panic_hook::set_once();

    // Open the image
    let img = imgutil::vec_to_image(input).unwrap();
    let img_rgb = img.to_rgb8();

    // Print out width and height
    println!("Dimensions: {:?}", img_rgb.dimensions());

    // Shrink image
    let img_rgb = canvas::shrink(img_rgb, 600);

    // Get a flat list of all pixels
    let pixels = img_rgb.pixels().collect::<Vec<_>>();
    println!("Total Pixels: {:?}", pixels.len());

    // Run K-means clustering to compute the dominant colors
    let centroids = kmeans(&pixels, k);
    println!("Centroids: {:?}", centroids.len());

    // Replace all pixels with the nearest centroid
    let img_rgb = canvas::recolor(img_rgb, &centroids);

    // Remove all areas that have less than the min defined area
    let img_rgb = canvas::denoise(img_rgb, min_area);

    // Scale up the image 4x
    let img_rgb = canvas::scale(img_rgb, 4);
    println!("Done flattening image!");

    // Convert the image to a vector of bytes
    let flat_img = imgutil::image_to_vec(&img_rgb, image::ImageFormat::Png);

    flat_img
}

pub fn flat_to_svg(file_name: &str, out_file_name: &str) {
    // Open the image
    let img = open(file_name).unwrap();
    let img_rgb = img.to_rgb8();

    // Convert the image to SVG
    let svg = svg::img_to_svg(&img_rgb);

    // Write the SVG to a file
    std::fs::write(out_file_name, svg).unwrap();
    println!("Done converting flat image to svg!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        // Variables!
        // let file_name = "/home/mikey/code/paint-by-numbers/pbn/tree.jpg";
        // let flat_file_name = "/home/mikey/code/paint-by-numbers/pbn/tree_paint.png";
        // let svg_file_name = "/home/mikey/code/paint-by-numbers/pbn/tree_paint.svg";
        let file_name = "/home/mikey/code/paint-by-numbers/pbn/hacker.jpg";
        let flat_file_name = "/home/mikey/code/paint-by-numbers/pbn/hacker.png";
        let svg_file_name = "/home/mikey/code/paint-by-numbers/pbn/hacker.svg";
        let k = 10;
        let min_area = 30;

        // img_to_flat(file_name, flat_file_name, k, min_area);
        // flat_to_svg(flat_file_name, svg_file_name);
    }
}

mod canvas;
mod kmeans;
mod svg;
mod imgutil;

use wasm_bindgen::prelude::*;
use kmeans::kmeans;

#[wasm_bindgen]
pub struct SvgData {
    svg: String,
    colors: Vec<String>,
}

impl SvgData {
    pub fn new(svg: String, colors: Vec<String>) -> SvgData {
        SvgData { svg, colors }
    }
}

#[wasm_bindgen]
impl SvgData {
    #[wasm_bindgen(getter)]
    pub fn svg(&self) -> String {
        self.svg.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn colors(&self) -> js_sys::Array {
        self.colors.iter().map(|c| JsValue::from(c.as_str())).collect()
    }
}

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
    let img = imgutil::vec_to_image(&input).unwrap();
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

#[wasm_bindgen]
pub fn flat_to_svg(input: Vec<u8>) -> SvgData {
    console_error_panic_hook::set_once();

    // Open the image
    let img = imgutil::vec_to_image(&input).unwrap();
    let img_rgb = img.to_rgb8();

    // Convert the image to SVG
    let (svg_data, colors) = svg::img_to_svg(&img_rgb);

    // Return the SVG data
    SvgData::new(svg_data, colors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        // Variables!
        let file_name = "./test/tree.jpg";
        let flat_file_name = "./test/tree_paint.png";
        let svg_file_name = "./test/tree_paint.svg";
        let color_file_name = "./test/tree_colors.txt";
        let k = 10;
        let min_area = 30;

        // Open image
        let img = image::open(file_name).unwrap();
        let img_rgb = img.to_rgb8();
        let buffer = imgutil::image_to_vec(&img_rgb, image::ImageFormat::Png);

        let out = img_to_flat(buffer, k, min_area);
        
        // Write image to file
        let img = imgutil::vec_to_image(&out).unwrap();
        img.save(flat_file_name).unwrap();

        // Convert to SVG
        let svg = flat_to_svg(out);
        std::fs::write(svg_file_name, svg.svg).expect("Unable to write file");
        std::fs::write(color_file_name, svg.colors.join("\n")).expect("Unable to write file");
    }

    #[test]
    fn test_flat_img() {
        // Variables!
        let file_name = "./test/tree.jpg";
        let flat_file_name = "./test/tree_paint.png";
        let k = 10;
        let min_area = 30;

        // Open image
        let img = image::open(file_name).unwrap();
        let img_rgb = img.to_rgb8();
        let buffer = imgutil::image_to_vec(&img_rgb, image::ImageFormat::Png);

        let out = img_to_flat(buffer, k, min_area);
        
        // Write image to file
        let img = imgutil::vec_to_image(&out).unwrap();
        img.save(flat_file_name).unwrap();
    }

    #[test]
    fn test_svg() {
        let file_name = "./test/tree_paint.png";
        let file_name = "./test/clouds_flat.png";
        let svg_file_name = "./test/tree_paint.svg";
        let color_file_name = "./test/tree_colors.txt";

        let img = image::open(file_name).expect("Run test_flat_img first");
        let img_rgb = img.to_rgb8();
        let buffer = imgutil::image_to_vec(&img_rgb, image::ImageFormat::Png);

        let svg = flat_to_svg(buffer);

        std::fs::write(svg_file_name, svg.svg).expect("Unable to write file");
        std::fs::write(color_file_name, svg.colors.join("\n")).expect("Unable to write file");
    }
}

mod kmeans;
mod canvas;
mod svg;

use image::open;
use kmeans::kmeans;

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

    img_to_flat(file_name, flat_file_name, k, min_area);
    flat_to_svg(flat_file_name, svg_file_name);
}

fn img_to_flat(file_name: &str, out_file_name: &str, k: i32, min_area: u32) {
    // Open the image
    let img = open(file_name).unwrap();
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

    // Save the new image
    img_rgb.save(out_file_name).unwrap();
}

fn flat_to_svg(file_name: &str, out_file_name: &str) {
    // Open the image
    let img = open(file_name).unwrap();
    let img_rgb = img.to_rgb8();

    // Convert the image to SVG
    let svg = svg::img_to_svg(&img_rgb);

    // Write the SVG to a file
    std::fs::write(out_file_name, svg).unwrap();
    println!("Done converting flat image to svg!");
}

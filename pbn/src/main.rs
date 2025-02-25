mod kmeans;
mod canvas;

use image::open;
use kmeans::kmeans;

fn main() {
    // Variables!
    let file_name = "/home/mikey/code/paint-by-numbers/pbn/tree.jpg";
    let out_file_name = "/home/mikey/code/paint-by-numbers/pbn/tree_paint.png";
    let k = 30;
    let min_area = 10;

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
    println!("Done!");

    // Save the new image
    img_rgb.save(out_file_name).unwrap();
}

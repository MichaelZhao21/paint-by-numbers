mod kmeans;
mod canvas;

use image::open;
use kmeans::kmeans;

fn main() {
    let img = open("/home/mikey/code/paint-by-numbers/pbn/tree.jpg").unwrap();
    let img_rgb = img.to_rgb8();

    // Shrink image
    let img_rgb = canvas::shrink(img_rgb, 600);

    // Print out width and height
    println!("Dimensions: {:?}", img_rgb.dimensions());
    
    // Get a flat list of all pixels
    let pixels = img_rgb.pixels().collect::<Vec<_>>();
    println!("Pixels: {:?}", pixels.len());

    // Run K-means clustering to compute the dominant colors
    let centroids = kmeans(&pixels, 10);
    println!("Centroids: {:?}", centroids.len());
    for i in 0..centroids.len() {
        println!("rgb({}, {}, {})", centroids[i][0], centroids[i][1], centroids[i][2]);
    }

    // Replace all pixels with the nearest centroid
    let img_rgb = canvas::recolor(img_rgb, &centroids);

    // Remove all areas that have less than the min defined area
    let img_rgb = canvas::denoise(img_rgb, 5);

    // Scale up the image 4x
    let img_rgb = canvas::scale(img_rgb, 4);
    println!("Done!");

    // Save the new image
    img_rgb.save("/home/mikey/code/paint-by-numbers/pbn/tree_paint.png").unwrap();
}

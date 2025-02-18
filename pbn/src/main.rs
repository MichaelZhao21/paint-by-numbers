mod kmeans;
mod canvas;

use image::open;
use kmeans::kmeans;

fn main() {
    let img = open("/home/mikey/code/numpainter/pbn/tree.jpg").unwrap();
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
    let new_img = canvas::recolor(img_rgb, &centroids);

    // Save the new image
    new_img.save("/home/mikey/code/numpainter/pbn/tree_paint.png").unwrap();
}

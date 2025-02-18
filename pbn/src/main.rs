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
    let mut new_pixels = Vec::new();
    for pixel in pixels.iter() {
        let mut min_distance = f64::MAX;
        let mut min_index = 0;
        for i in 0..centroids.len() {
            let d = kmeans::distance(pixel, &centroids[i]);
            if d < min_distance {
                min_distance = d;
                min_index = i;
            }
        }
        new_pixels.push(centroids[min_index]);
    }

    // Create a new image with the new pixels
    let new_img = image::ImageBuffer::from_fn(img_rgb.width(), img_rgb.height(), |x, y| {
        new_pixels[(y * img_rgb.width() + x) as usize]
    });

    // Save the new image
    new_img.save("/home/mikey/code/numpainter/pbn/tree_paint.jpg").unwrap();
}

use std::{
    cell::RefCell,
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::kmeans;
use image::{Rgb, RgbImage};

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
    println!("Shrinking image to {}x{}...", new_size.0, new_size.1);

    return image::imageops::resize(
        &img,
        new_size.0,
        new_size.1,
        image::imageops::FilterType::Lanczos3,
    );
}

pub fn scale(img: RgbImage, scale: u32) -> RgbImage {
    println!("Scaling image up by {}...", scale);
    let (width, height) = img.dimensions();
    let new_size = (width * scale, height * scale);
    return image::imageops::resize(
        &img,
        new_size.0,
        new_size.1,
        image::imageops::FilterType::Nearest,
    );
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

/// Remove all areas in an image that have less than the min defined area
pub fn denoise(img: RgbImage, min_area: u32) -> RgbImage {
    // 1. Keep track of visited pixels
    // 2. Loop through grid top to bottom, left to right
    // 3. Each time we encounter a cell that isn't visited do the following:
    // 	1. Create a set called **edge set**
    // 	2. Run flood fill, counting the number of cells visited and visiting the cells
    // 	3. At the edges, add to **edge set**
    // 	4. Figure out the new color based on the most common color in the edge set
    // 	5. Add all edges in edge set to a queue, iterate through them:
    // 		1. If the edge does not match the new color, ignore it and continue
    // 		2. If the edge is VISITED and matches the new color, "expand" the current area, increase the area count by **the threshold**, and remove them from the edge set (the logic here is that if an edge is visited, it means that it already larger than the min area and thus will make the current area larger than the min area; ie. the visited edge will be a part of a group of colors that is at least the size of the threshold)
    // 		3. If the edge is NOT VISITED and matches the new color, "expand" the current area, visiting those nodes, increasing the area count, and removing them from the edge set
    // 		4. Add all of the edges' unvisited children to the queue
    // 	7. If the area is above the threshold or no more edges remain in the edge set, exit
    // 	8. Otherwise, repeat from step 5

    // Create a new image to store the denoised image
    // Need to use Rc and RefCell to allow for mutable and immutable borrows (Sometimes i fuckin hate rust)
    let new_img_ref = Rc::new(RefCell::new(img.clone()));

    let mut visited = vec![vec![false; img.height() as usize]; img.width() as usize];

    // Loop through the image's pixels
    for loop_y in 0..img.height() {
        for loop_x in 0..img.width() {
            // Ignore all cells that are visited
            if visited[loop_x as usize][loop_y as usize] {
                continue;
            }
            let mut curr_visited = HashSet::<(usize, usize)>::new();
            let mut color;

            // Block for immutable borrow
            {
                let new_img = new_img_ref.borrow();

                // Initialize values required for denoising the current region
                // We define area separately from curr_visited because of the case of expanding to valid sized areas
                let mut edge_set = HashSet::<(usize, usize)>::new();
                let mut area = 0;
                color = new_img.get_pixel(loop_x, loop_y).clone();

                // Run flood fill
                let mut queue = VecDeque::<(usize, usize)>::new();
                queue.push_back((loop_x as usize, loop_y as usize));
                while !queue.is_empty() {
                    // Get the next pixel (unwrap bc of is_empty check)
                    let (x, y) = queue.pop_front().unwrap();

                    // Ignore if in visited in the current flood fill
                    if curr_visited.contains(&(x, y)) {
                        continue;
                    }

                    // Add to edge set if visited or not the same color
                    if visited[x][y] {
                        // If it matches the current color, then that means that the new area will be > min_area
                        // as all visited pixels are part of an area that is > min_area
                        // However, thinking about it, this case should never happen...
                        if *new_img.get_pixel(x as u32, y as u32) == color {
                            println!("Edge pixel is same color... this shouldn't happen oopsie");
                            area += min_area;
                        }
                        edge_set.insert((x, y));
                        continue;
                    } else if *new_img.get_pixel(x as u32, y as u32) != color {
                        edge_set.insert((x, y));
                        continue;
                    }

                    // Visit the pixel
                    curr_visited.insert((x, y));
                    area += 1;

                    // Add all children to the queue
                    for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        // Only add if in bounds
                        if nx < 0 || ny < 0 || nx >= img.width() as i32 || ny >= img.height() as i32
                        {
                            continue;
                        }
                        queue.push_back((nx as usize, ny as usize));
                    }
                }

                println!("Edge set: {:?}", edge_set);
                println!("Area after initial: {}", area);

                // If the area is below the threshold, we need to expand our area
                let mut new_color_items = HashSet::<(usize, usize)>::new();
                while area < min_area {
                    // If there are no more edges, we can't expand anymore
                    if edge_set.is_empty() {
                        break;
                    }

                    // Reset new_color_items
                    new_color_items.clear();

                    // Get the new color as the most common color in the edge set
                    // Unwrap for max color because we know there is at least one color in the edge set
                    let mut color_counts = HashMap::<&Rgb<u8>, u32>::new();
                    for (x, y) in edge_set.iter() {
                        let color = new_img.get_pixel(*x as u32, *y as u32);
                        *color_counts.entry(color).or_insert(0) += 1;
                    }
                    println!("Color counts: {:?}", color_counts);
                    let new_color = *color_counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
                    println!("New color: {:?}", new_color);

                    // Check every edge node, running flood fill on any that are the same color and not visited
                    let mut queue = VecDeque::from_iter(edge_set.clone());
                    while !queue.is_empty() {
                        // Get the next pixel (unwrap bc of is_empty check)
                        let (x, y) = queue.pop_front().unwrap();

                        // Ignore if the wrong color
                        if new_img.get_pixel(x as u32, y as u32) != new_color {
                            continue;
                        }

                        // If the edge is already visited
                        if visited[x][y] {
                            area += min_area;
                            edge_set.remove(&(x, y));
                            continue;
                        }

                        // Otherwise, if node has been visited in this flood fill, ignore
                        if curr_visited.contains(&(x, y)) {
                            continue;
                        }

                        // Otherwise, visit node, increase area, and add children to queue
                        curr_visited.insert((x, y));
                        area += 1;

                        // Add all children to the queue
                        for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            // Only add if in bounds
                            if nx < 0
                                || ny < 0
                                || nx >= img.width() as i32
                                || ny >= img.height() as i32
                            {
                                continue;
                            }
                            queue.push_back((nx as usize, ny as usize));
                        }

                        // Remove from edge set
                        edge_set.remove(&(x, y));
                    }

                    color = *new_color;
                }
                println!("Color: {:?}", color);

                println!("Visited: {:?}", curr_visited.len());
                println!("Area after edge fill: {:?}", area);
                println!("==============================================\n");
            }

            // Visit all pixels in the area and color them
            {
                let mut new_img_mut = new_img_ref.borrow_mut();
                for (x, y) in curr_visited.iter() {
                    // Ignore everything that's already colored correctly
                    // if new_color_items.contains(&(*x, *y)) {
                    //     continue;
                    // }

                    visited[*x][*y] = true;
                    new_img_mut.put_pixel(*x as u32, *y as u32, color);
                }
            }
        }
    }

    let out = new_img_ref.borrow().clone();
    out
}

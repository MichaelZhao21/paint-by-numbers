use std::collections::VecDeque;

use image::{Rgb, RgbImage};
use std::cmp;

/// Convert a flat image to an SVG string.
/// Returns the SVG string and a list of colors used in the image.
pub fn img_to_svg(img: &RgbImage) -> (String, Vec<String>) {
    println!("Converting image to SVG...");
    let mut out = String::with_capacity(1000);

    // Get image dimensions
    let (width, height) = img.dimensions();

    // SVG Header
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\">\n",
        width, height
    ));

    // Create map of nums to colors
    let mut color_map = std::collections::HashMap::<&Rgb<u8>, u32>::new();
    let mut color_list = Vec::<String>::new();
    let mut color_count = 0;
    for y in 0..height {
        for x in 0..width {
            let color = img.get_pixel(x, y);
            if !color_map.contains_key(&color) {
                color_count += 1;
                color_map.insert(color, color_count);
                color_list.push(rgb_to_hex(color));
            }
        }
    }

    // Draw borders and numbers
    // Loop through all pixels
    let mut visited = vec![vec![false; img.height() as usize]; img.width() as usize];
    let mut area_count = 0;
    for y in 0..img.height() {
        for x in 0..img.width() {
            // If visited, ignore
            if visited[x as usize][y as usize] {
                continue;
            }

            // Find the borders of the current area
            let borders = find_area_borders(img, &mut visited, x, y);

            // If the borders are empty, ignore
            if borders.is_empty() {
                println!(
                    "WARNING: Could not find borders for area with ({}, {})",
                    x, y
                );
                continue;
            }

            // Get the position of the number
            let (nx, ny) = get_num_pos(&borders, img.height() as usize);

            // Optimize the borders
            let borders = borders
                .iter()
                .map(|b| optimize_border(b.clone()))
                .collect::<Vec<_>>();

            // Write the borders to SVG
            out.push_str(&format!("<path stroke=\"black\" fill=\"transparent\" stroke-width=\"1\" id=\"shape-{}\" fill-rule=\"evenodd\" class=\"unfilled\" d=\"", area_count));
            for border in borders.iter() {
                out.push_str(&format!(" M{} {}", border[0].0, border[0].1));
                for (x, y) in border.iter().skip(1) {
                    out.push_str(&format!(" L {} {}", x, y));
                }
                out.push_str(" Z");
            }
            out.push_str("\" />\n");

            // Draw the number
            let col_index = color_map.get(&img.get_pixel(x, y)).unwrap();
            out.push_str(&format!(
                "<text id=\"label-{}\" x=\"{}\" y=\"{}\" font-size=\"10\">{}</text>\n",
                area_count, nx, ny, col_index
            ));

            // Increment the area count
            area_count += 1;
        }
    }

    // SVG Footer
    out.push_str("</svg>\n");

    (out, color_list)
}

/// Convert an RGB color to a hex string.
fn rgb_to_hex(rgb: &Rgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

/// Find the borders of an area, returning a list of list of border points.
/// The list of border points represents the outer border. Subsequent lists
/// represent holes in the area.
fn find_area_borders(
    img: &RgbImage,
    visited: &mut Vec<Vec<bool>>,
    ox: u32,
    oy: u32,
) -> Vec<Vec<(usize, usize)>> {
    let mut border = Vec::<Vec<(usize, usize)>>::new();

    // Variables for flood filling
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back((ox as usize, oy as usize));
    let color = img.get_pixel(ox, oy);

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        // If visited, ignore
        if visited[x][y] {
            continue;
        }

        // If wrong color, ignore
        if img.get_pixel(x as u32, y as u32) != color {
            continue;
        }

        // Mark as visited
        visited[x][y] = true;

        // Check if the pixel is a border pixel
        if is_border(img, x, y) {
            // If it is, follow the border
            let b = follow_edge(img, visited, &mut queue, x, y);
            border.push(b);
        }

        // Flood fill in the 4 cardinal directions
        for (dx, dy) in DIRECTIONS.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Only add if in bounds
            if nx < 0 || ny < 0 || nx >= img.width() as i32 || ny >= img.height() as i32 {
                continue;
            }

            queue.push_back((nx as usize, ny as usize));
        }
    }

    border
}

fn is_border(img: &RgbImage, x: usize, y: usize) -> bool {
    // Check if the pixel is on the edge of the image
    if x == 0 || y == 0 || x == img.width() as usize - 1 || y == img.height() as usize - 1 {
        return true;
    }

    // Check the 8 pixels around the current pixel and see if it's a different color
    let color = img.get_pixel(x as u32, y as u32);
    let mut different = false;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // If out of bounds, assume it's a border
            if nx < 0 || ny < 0 || nx >= img.width() as i32 || ny >= img.height() as i32 {
                different = true;
                break;
            }

            let ncolor = img.get_pixel(nx as u32, ny as u32);
            if ncolor != color {
                different = true;
                break;
            }
        }
    }

    different
}

// 4 cardinal directions in a clockwise order
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// Follow the edge of an area starting at a given point.
/// This function will return a list of points that represent the border of the area,
/// stopping when it reaches the starting point.
/// We assume that each pixel can only touch 1 border.
fn follow_edge(
    img: &RgbImage,
    visited: &mut Vec<Vec<bool>>,
    queue: &mut VecDeque<(usize, usize)>,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    let mut current = (x, y);
    let mut parent = (x, y);
    let mut border = Vec::<(usize, usize)>::new();
    let mut found;
    border.push(current);
    let in_color = img.get_pixel(x as u32, y as u32);

    loop {
        // Mark the current pixel as visited
        visited[current.0][current.1] = true;

        // Check the 4 cardinal directions
        found = false;
        for d in DIRECTIONS {
            let nx = current.0 as i32 + d.0;
            let ny = current.1 as i32 + d.1;

            // println!("Checking ({:?}, {:?}) for d = {:?}", nx, ny, d);

            // If the new pixel is the starting pixel, we are done
            if (nx as usize, ny as usize) == (x, y) {
                return border;
            }

            // Check if the new pixel is out of bounds
            if nx < 0 || ny < 0 || nx >= img.width() as i32 || ny >= img.height() as i32 {
                continue;
            }

            // Check to make sure we don't go backwards
            if parent == (nx as usize, ny as usize) {
                continue;
            }

            // Check if the new pixel is the same color
            let ncolor = img.get_pixel(nx as u32, ny as u32);
            if ncolor != in_color {
                continue;
            }

            // If the new pixel is not a border, continue
            // Also add it to the flood fill queue
            if !is_border(img, nx as usize, ny as usize) {
                queue.push_back((nx as usize, ny as usize));
                continue;
            }

            // If the new pixel is a border, add it to the border list, mark it as visited
            // and set the new pixel as the current pixel
            parent = current;
            current = (nx as usize, ny as usize);
            border.push(current);
            visited[nx as usize][ny as usize] = true;
            found = true;
            // println!("Adding {:?}", current);
            break;
        }

        // If we reach here and haven't found anything, we have a problem
        if !found {
            println!(
                "WARNING: Area is not closed at {:?} (cannot find border to follow; this should not happen...)",
                current
            );
            return border;
        }
    }
}

/// Get the position of the number for a given area.
/// The function will return the position of the number as a tuple of (x, y).
fn get_num_pos(border_list: &Vec<Vec<(usize, usize)>>, max_height: usize) -> (usize, usize) {
    // Get the first border
    let outer_border = &border_list[0];

    // Find the centroid of the outer polygon
    let mut x_sum = 0;
    let mut y_sum = 0;
    for (x, y) in outer_border {
        x_sum += x;
        y_sum += y;
    }
    let mut centroid = (x_sum / outer_border.len(), y_sum / outer_border.len());

    // Flatten all borders into one
    let borders = border_list
        .iter()
        .flatten()
        .collect::<Vec<&(usize, usize)>>();

    // If centroid is on a border, move it in one of the directions that isn't a border
    if borders.contains(&&centroid) {
        let mut new_centroid = centroid;
        for (dx, dy) in DIRECTIONS.iter() {
            let nx = centroid.0 as i32 + dx;
            let ny = centroid.1 as i32 + dy;
            if !borders.contains(&&(nx as usize, ny as usize)) {
                new_centroid = (nx as usize, ny as usize);
                break;
            }
        }
        if centroid == new_centroid {
            panic!("Could not find a new centroid");
        }
        centroid = new_centroid;
    }

    // Count the number of times a ray to the positive x direction intersects the borders
    let border_hits = borders
        .iter()
        .filter(|&b| (*b).1 == centroid.1 && (*b).0 > centroid.0)
        .count();
    let inside = border_hits % 2 == 1;

    // Loop through list of borders and find borders in the 4 directions
    let mut left = Vec::<(usize, usize)>::new();
    let mut right = Vec::<(usize, usize)>::new();
    let mut up = Vec::<(usize, usize)>::new();
    let mut down = Vec::<(usize, usize)>::new();

    for border in border_list.iter() {
        for (x, y) in border {
            if *y == centroid.1 {
                if *x < centroid.0 {
                    left.push((*x, *y));
                } else {
                    right.push((*x, *y));
                }
            }
            if *x == centroid.0 {
                if *y < centroid.1 {
                    up.push((*x, *y));
                } else {
                    down.push((*x, *y));
                }
            }
        }
    }

    // Sort all lists by distance to centroid
    left.sort_by(|a, b| b.0.cmp(&a.0));
    right.sort_by(|a, b| a.0.cmp(&b.0));
    up.sort_by(|a, b| b.1.cmp(&a.1));
    down.sort_by(|a, b| a.1.cmp(&b.1));

    // Make list of possible points
    let mut possible_points = Vec::<((usize, usize), (usize, usize))>::new();

    if inside {
        // If inside, take first point from left and right, then up and down
        if left.len() > 0 && right.len() > 0 {
            possible_points.push((left[0], right[0]));
        }
        if up.len() > 0 && down.len() > 0 {
            possible_points.push((up[0], down[0]));
        }
    } else {
        // Otherwise, take two points from each side
        if left.len() > 1 {
            possible_points.push((left[0], left[1]));
        }
        if right.len() > 1 {
            possible_points.push((right[0], right[1]));
        }
        if up.len() > 1 {
            possible_points.push((up[0], up[1]));
        }
        if down.len() > 1 {
            possible_points.push((down[0], down[1]));
        }
    }

    // println!("Centroid: {:?}", centroid);
    // println!("Possible Points: {:?}", possible_points);

    // Find the pair of points that have the largest distance between them
    let mut max_dist = 0;
    let mut max_pair = (centroid, centroid);
    for (p1, p2) in possible_points {
        let dist = (p1.0 as i32 - p2.0 as i32).pow(2) + (p1.1 as i32 - p2.1 as i32).pow(2);
        if dist > max_dist {
            max_dist = dist;
            max_pair = (p1, p2);
        }
    }

    // println!("Max Pair: {:?}", max_pair);

    // Return the midpoint of the pair of points
    let d = 2;
    let nx = cmp::max((max_pair.0 .0 + max_pair.1 .0) / 2 - d, d) as usize;
    let ny = cmp::min((max_pair.0 .1 + max_pair.1 .1) / 2 + d, max_height - d) as usize;
    // println!("Point: {:?}", (nx, ny));
    // println!("==========================================================");

    (nx, ny)
}

fn optimize_border(border: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if border.len() < 3 {
        return border;
    }

    let mut optimized = Vec::<(usize, usize)>::new();
    optimized.push(border[0]);
    let mut dir = (0, 0);
    let mut curr = border[0];

    // Loop through the border in order
    for b in border.iter().skip(1) {
        // Check direction of next border
        let next_dir = (b.0 as i32 - curr.0 as i32, b.1 as i32 - curr.1 as i32);

        // If direction is in the same direction, continue
        // Current item part of the same current border line
        if dir == (0, 0) || dir == next_dir {
            dir = next_dir;
            curr = *b;
            continue;
        }

        // Otherwise, that means that we've changed directions
        // Add the "curr" point to the list and update the direction
        optimized.push(curr);
        dir = next_dir;
        curr = *b;
    }

    optimized
}

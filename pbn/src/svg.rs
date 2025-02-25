use std::collections::VecDeque;

use image::{Rgb, RgbImage};

pub fn img_to_svg(img: &RgbImage) -> String {
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
    let mut color_count = 0;
    for y in 0..height {
        for x in 0..width {
            let color = img.get_pixel(x, y);
            if !color_map.contains_key(&color) {
                color_count += 1;
                color_map.insert(color, color_count);
            }
        }
    }

    // Draw borders and numbers
    // Loop through all pixels
    let mut visited = vec![vec![false; img.height() as usize]; img.width() as usize];
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
                println!("WARNING: Area is empty at ({}, {})", x, y);
                continue;
            }

            // Write the borders to SVG
            out.push_str("<path stroke=\"white\" fill=\"transparent\" stroke-width=\"1\" d=\"");
            for border in borders.iter() {
                out.push_str(&format!(" M{} {}", border[0].0, border[0].1));
                for (x, y) in border.iter().skip(1) {
                    out.push_str(&format!(" L {} {}", x, y));
                }
                out.push_str(" Z");
            }
            out.push_str("\" />\n");

            // Draw the number
            let (nx, ny) = get_num_pos(&borders[0]);
            let col_index = color_map.get(&img.get_pixel(x, y)).unwrap();
            out.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"Verdana\" font-size=\"10\" fill=\"red\">{}</text>\n",
                nx, ny, col_index
            ));
        }
    }

    // SVG Footer
    out.push_str("</svg>\n");
    out
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

            // Check if the new pixel is visited
            if visited[nx as usize][ny as usize] {
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
                "WARNING: Area is not closed at {:?} (cannot find border to follow)",
                current
            );
            return border;
            // panic!("Area is not closed");
        }
    }
}

fn get_num_pos(border: &Vec<(usize, usize)>) -> (usize, usize) {
    // Find the center of the border
    let mut x_sum = 0;
    let mut y_sum = 0;
    for (x, y) in border {
        x_sum += x;
        y_sum += y;
    }
    let x_center = x_sum / border.len();
    let y_center = y_sum / border.len();

    return (x_center, y_center);
}

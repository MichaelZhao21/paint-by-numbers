use image::Rgb;
use rand::{distr::{weighted::WeightedIndex, Distribution}, Rng};

const MAX_ITER: i32 = 100;

pub fn kmeans(arr: &Vec<&Rgb<u8>>, k: i32) -> Vec<Rgb<u8>> {
    println!("Running K-means with k={k}...");
    let mut centroids = Vec::<Rgb<u8>>::new();

    // Clone arr
    let mut not_picked = arr.clone();

    // Pick first centroid randomly
    // We will run the K-means++ algorithm for initial centroid selection
    let first_index = rand::rng().random_range(..arr.len());
    centroids.push(*not_picked.remove(first_index));

    // Loop until we have k centroids
    for _ in 1..k {
        // Calculate the distance from each point to the nearest centroid
        // and store the minimum distance squared
        let mut distances = Vec::<f64>::new();
        for point in not_picked.iter() {
            let mut min_distance = f64::MAX;
            for centroid in centroids.iter() {
                let d = distance(point, centroid);
                if d < min_distance {
                    min_distance = d;
                }
            }
            distances.push(min_distance*min_distance);
        }

        // Created a weighted probability distribution based on the distances
        let dist = WeightedIndex::new(&distances).unwrap();
        let mut rng = rand::rng();
        let index = dist.sample(&mut rng);
        centroids.push(*not_picked.remove(index));
    }

    // We will now run the K-means algorithm
    let mut changed = true;
    let mut iter = 0;
    while changed && iter < MAX_ITER {
        changed = false;

        // Create clusters from the centroids
        let mut clusters = vec![Vec::<&Rgb<u8>>::new(); k as usize];

        // Assign each point to the nearest centroid
        for point in arr.iter() {
            let mut min_distance = f64::MAX;
            let mut min_index = 0;
            for (i, centroid) in centroids.iter().enumerate() {
                let d = distance(point, centroid);
                if d < min_distance {
                    min_distance = d;
                    min_index = i;
                }
            }
            clusters[min_index].push(point);
        }

        // Calculate the new centroids
        for (i, cluster) in clusters.iter().enumerate() {
            // If the cluster is empty, pick the point furthest from all centroids
            if cluster.is_empty() {
                let mut max_distance = 0.0;
                let mut max_index = 0;
                for (j, point) in arr.iter().enumerate() {
                    let mut min_distance = f64::MAX;
                    for centroid in centroids.iter() {
                        let d = distance(point, centroid);
                        if d < min_distance {
                            min_distance = d;
                        }
                    }
                    if min_distance > max_distance {
                        max_distance = min_distance;
                        max_index = j;
                    }
                }
                centroids[i] = *arr[max_index];
                changed = true;
                continue;
            }

            // Calculate the new centroid
            let new_centroid = re_centroid(cluster);
            if centroids[i] != new_centroid {
                centroids[i] = new_centroid;
                changed = true;
            }
        }

        iter += 1;
        // println!("KMeans Iteration: {}", iter);
    }

    return centroids;
}

/// Compute the "distance" between two colors
pub fn distance(a: &Rgb<u8>, b: &Rgb<u8>) -> f64 {
    let r = a[0] as f64 - b[0] as f64;
    let g = a[1] as f64 - b[1] as f64;
    let b = a[2] as f64 - b[2] as f64;

    return (r * r + g * g + b * b).sqrt();
}

/// Calculate the centroid of a cluster based on
/// the average of all points in the cluster
fn re_centroid(cluster: &Vec<&Rgb<u8>>) -> Rgb<u8> {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for point in cluster.iter() {
        r += point[0] as i32;
        g += point[1] as i32;
        b += point[2] as i32;
    }
    let len = cluster.len() as i32;
    if len > 0 {
        r /= len;
        g /= len;
        b /= len;
    }
    return Rgb([r as u8, g as u8, b as u8]);
}

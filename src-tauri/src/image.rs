use crate::{filesystem::load_image, ResultExtended, Star};

use image::imageops;
use image::io::Reader as ImageReader;

use rand::seq::SliceRandom;

fn find_closest(center: &[(f64, f64, f64)], (x, y, z): (f64, f64, f64)) -> usize {
    center
        .iter()
        .map(|(cx, cy, cz)| (x - cx).powi(2) + (y - cy).powi(2) + (z - cz).powi(2))
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            // println!("{}, {}", a, b);
            a.partial_cmp(b).unwrap()
        })
        .unwrap()
        .0
}

fn k_means<T>(colors: &Vec<((f64, f64, f64), T)>) -> Vec<(&T, usize)> {
    // We assume a group size of 24 (the number of colors)
    const CLUSTER_AMOUNT: usize = 24;
    const ITERATION_AMOUNT: usize = 20;

    // create group locations
    let mut rng = rand::thread_rng();
    let mut cluster_centers: Vec<(f64, f64, f64)> = colors
        .choose_multiple(&mut rng, CLUSTER_AMOUNT)
        .map(|(pos, _data)| pos)
        .cloned()
        .collect();

    for _ in 0..ITERATION_AMOUNT {
        // Calulcate current members
        let mut cluster_members: Vec<Vec<&(f64, f64, f64)>> = vec![vec![]; CLUSTER_AMOUNT];
        for (color, _other) in colors.iter() {
            let index = find_closest(&cluster_centers, *color);
            cluster_members[index].push(color);
        }

        // Calculate new centers
        for (index, members) in cluster_members.into_iter().enumerate() {
            let len = members.len() as f64;

            if len == 0.0 {
                continue;
            }

            let pos = members
                .into_iter()
                .copied()
                .reduce(|(x1, y1, z1), (x2, y2, z2)| (x1 + x2, y1 + y2, z1 + z2))
                .unwrap_or((0.0, 0.0, 0.0));
            cluster_centers[index] = (pos.0 / len, pos.1 / len, pos.2 / len);
        }
    }

    colors
        .iter()
        .map(|(color, data)| (data, find_closest(&cluster_centers, *color)))
        .collect()
}

#[tauri::command(async)]
pub fn load_image_colors(window: tauri::Window) -> Result<Vec<Star>, String> {
    let path = load_image(window)?;
    let image = ImageReader::open(path)
        .prefix_error("error reading file")?
        .decode()
        .prefix_error("error decoding image")?;

    let height = image.height();
    let width = image.width();
    let scale = (((height as f64).sqrt() * (width as f64).sqrt()) / 95.0).ceil() as u32;
    let scale = scale.max(1);

    let image = imageops::resize(&image, width / scale, height / scale, imageops::Nearest);

    let mut scale: f32 = 0.0;
    let mut stars: Vec<Star> = Vec::with_capacity((image.width() * image.height()) as usize);

    for (&(x, y, alpha), color) in k_means(
        &image
            .enumerate_pixels()
            .map(|(x, y, &image::Rgba([r, g, b, alpha]))| {
                ((r as f64, g as f64, b as f64), (x, y, alpha))
            })
            .collect(),
    ) {
        if alpha == 0 {
            continue;
        }

        let x = x as f32;
        let y = y as f32;
        let color = color as u8 + 1;

        stars.push(Star { x, y, color });

        scale = scale.max(x / 1000.0).max(y / 500.0);
    }

    let stars = stars
        .into_iter()
        .map(|Star { x, y, color }| Star {
            x: x / scale,
            y: y / scale,
            color,
        })
        .collect();

    Ok(stars)
}

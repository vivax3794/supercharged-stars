use crate::{filesystem::load_image, ResultExtended, Star};

use image::imageops;
use image::io::Reader as ImageReader;

#[tauri::command(async)]
pub fn load_image_colors(window: tauri::Window) -> Result<Vec<Star>, String> {
    let path = load_image(window)?;
    let image = ImageReader::open(path)
        .prefix_error("error reading file")?
        .decode()
        .prefix_error("error decoding image")?;

    let height = image.height();
    let width = image.width();
    let scale = (((height as f32).sqrt() * (width as f32).sqrt()) / 95.0).ceil() as u32;
    let scale = scale.max(1);

    let image = imageops::resize(&image, width / scale, height / scale, imageops::Nearest);

    let mut scale: f32 = 0.0;
    let mut stars: Vec<Star> = Vec::with_capacity((image.width() * image.height()) as usize);

    for (x, y, pixel) in image.enumerate_pixels() {
        let x = x as f32;
        let y = y as f32;

        let [r, g, b, alpha] = pixel.0;
        let r = r as f32;
        let g = g as f32;
        let b = b as f32;

        if alpha == 0 {
            continue;
        }

        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let hue = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        } * 60.0;
        let hue = if hue < 0.0 { hue + 360.0 } else { hue };

        let color = (hue / 360.0 * 24.0).round() as u8 + 1;
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

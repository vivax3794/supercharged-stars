//! Tauri commands related to reading/writting stars from/to disk

use crate::{ResultExtended, Star};

/// The arcade cli often stored csv files in the 0-1 range.
/// In this new tool we have decided to store in the 0-1000 0-500 range
/// Hopefully this will make working with stars more consistent!
fn is_likely_old_format(stars: &[Star]) -> bool {
    stars.iter().all(|Star { x, y, .. }| *x <= 1.0 && *y <= 1.0)
}

/// Load stars from disk using a file dialog
#[tauri::command(async)]
pub fn load_stars(window: tauri::Window) -> Result<(Vec<Star>, Option<String>), String> {
    use tauri::api::dialog::blocking as dialog;

    let path = dialog::FileDialogBuilder::new()
        .set_parent(&window)
        .set_title("Stars to load")
        .add_filter("CSV", &["csv"])
        .pick_file()
        .ok_or("Error getting file path")?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .prefix_error("error parsing file")?;

    let mut stars = Vec::new();
    for record in reader.deserialize::<Star>() {
        let star = record.prefix_error("error parsing file")?;
        stars.push(star);
    }

    if is_likely_old_format(&stars) {
        Ok((
            stars
                .into_iter()
                .map(|Star { x, y, color }| Star {
                    x: x * 1000.0,
                    y: y * 500.0,
                    color,
                })
                .collect(),
            Some(String::from("Converted file from 0-1 format.")),
        ))
    } else {
        Ok((stars, None))
    }
}

/// Save stars to disk
#[tauri::command(async)]
pub fn save_stars(window: tauri::Window, stars: Vec<Star>) -> Result<(), String> {
    use tauri::api::dialog::blocking as dialog;

    let path = dialog::FileDialogBuilder::new()
        .set_parent(&window)
        .set_title("Stars to save")
        .add_filter("CSV", &["csv"])
        .set_file_name("stars.csv")
        .save_file()
        .ok_or("Error getting file path")?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(path)
        .prefix_error("error writting file")?;

    for star in stars.into_iter() {
        writer.serialize(star).prefix_error("error writting file")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_trigger_on_clearly_new_format() {
        let stars = vec![
            Star {
                x: 0.5,
                y: 0.2,
                color: 0,
            },
            Star {
                x: 1000.0,
                y: 10000.0,
                color: 1,
            },
        ];
        assert_eq!(is_likely_old_format(&stars), false);
    }

    #[test]
    fn does_trigger_on_old_format() {
        let stars = vec![
            Star {
                x: 0.5,
                y: 0.2,
                color: 0,
            },
            Star {
                x: 0.1,
                y: 0.89,
                color: 1,
            },
        ];
        assert_eq!(is_likely_old_format(&stars), true);
    }
}

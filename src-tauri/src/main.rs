#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Star {
    x: f32,
    y: f32,
    color: u8,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_stars])
        .setup(|app| {
            let handle = app.handle();

            tauri::async_runtime::spawn(async move {
                match handle.updater().unwrap().check().await {
                    Ok(update) => {
                        update.download_and_install().await;
                    }

                    Err(e) => {
                        println!("ERROR: {}", e);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
fn load_stars(window: tauri::Window) -> Result<Vec<Star>, String> {
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
        .or_else(|err| Err(format!("error parsing file {:#?}", err)))?;

    let mut stars = Vec::new();
    for record in reader.deserialize::<Star>() {
        let star = record.or_else(|err| Err(format!("error parsing file {:#?}", err)))?;
        stars.push(Star {
            x: star.x * 1000.0,
            y: star.y * 500.0,
            ..star
        });
    }

    Ok(stars)
}

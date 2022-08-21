#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app::filesystem::load_stars,
            app::filesystem::save_stars,
            app::api::send_stars,
            app::image::load_image_colors,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


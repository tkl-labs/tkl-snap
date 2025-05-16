pub mod capture;
use tauri::Manager;

use crate::capture::{get_image_inner, CaptureError, GlobalState};

use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app| {
        app.manage(Mutex::new(GlobalState::default()));
        Ok(())

    })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_image() -> Result<String, CaptureError> {
    get_image_inner()
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

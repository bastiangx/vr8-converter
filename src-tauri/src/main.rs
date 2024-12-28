// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// DO NOT DELETE CFG LINE BELOW
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod converter;
use converter::{ConversionResult, Converter};
use std::fs;
use std::path::Path;
use tauri::Manager;
use tauri_plugin_dialog::init as init_dialog;

#[tauri::command]
async fn convert_files(paths: Vec<String>, _is_dir: bool) -> Result<ConversionResult, String> {
    let converter = Converter::default();

    let first_path = Path::new(&paths[0]);
    let parent_dir = first_path.parent().unwrap_or(Path::new("."));
    let output_dir = parent_dir.join("wav-converted");

    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    let mut all_files = Vec::new();
    for path in paths {
        let path = Path::new(&path);
        let file = fs::read_dir(path.parent().unwrap_or(Path::new(".")))
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .find(|entry| entry.path() == path)
            .ok_or_else(|| "error: file not found".to_string())?;

        all_files.push(file);
    }

    converter.process_files(all_files, &output_dir)
        .map_err(|e| e.to_string())
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(init_dialog())
        .invoke_handler(tauri::generate_handler![convert_files])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(|event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    std::process::exit(0);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

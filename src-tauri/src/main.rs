// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// DO NOT DELETE CFG LINE BELOW
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod converter;
use converter::{ConversionResult, Converter};
use std::fs;
use std::path::Path;
use tauri_plugin_dialog::init as init_dialog;

#[tauri::command]
async fn convert_files(path: String, is_dir: bool) -> Result<ConversionResult, String> {
    let converter = Converter::default();
    let input_path = Path::new(&path);

    let output_dir = if is_dir {
        input_path.join("wav-files")
    } else {
        input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("wav-files")
    };

    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    let files: Vec<_> = if is_dir {
        fs::read_dir(input_path)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("VR8"))
            .collect()
    } else {
        vec![fs::read_dir(input_path.parent().unwrap_or(Path::new(".")))
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .find(|entry| entry.path() == input_path)
            .ok_or_else(|| "File not found".to_string())?]
    };
    let converted = converter
        .process_files(files, &output_dir)
        .map_err(|e| e.to_string())?;

    Ok(converted)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(init_dialog())
        .invoke_handler(tauri::generate_handler![convert_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

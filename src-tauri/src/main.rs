// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// DO NOT DELETE CFG LINE BELOW
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_dialog::init as init_dialog;
use vr8_core::{convert_files, ConversionResult};

/// Command exposed to the front for conversion
/// Takes paths to files and an output directory, returns conversion results
#[tauri::command]
async fn convert_files_command(
    window: tauri::Window,
    paths: Vec<String>,
    output_dir: String,
) -> Result<ConversionResult, String> {
    convert_files(Some(window), paths, output_dir).await
}

/// Main entry point for the Tauri desktop application
fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(init_dialog())
        // Register the conversion command
        .invoke_handler(tauri::generate_handler![convert_files_command])
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

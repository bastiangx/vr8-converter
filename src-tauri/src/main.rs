// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// DO NOT DELETE CFG LINE BELOW
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Main Tauri application entrypoint for VR8 Converter
/// Handles file conversion from .VR8 to .WAV and manages the application window
mod converter;
use converter::{ConversionResult, Converter};
use tauri::Manager;
use tauri_plugin_dialog::init as init_dialog;

/// Converts selected VR8 file(s) to WAV format
///
/// # Arguments
/// * `window` - Tauri window instance for progress updates
/// * `paths` - Vector of file paths to convert
/// * `output_dir` - Base directory where converted files will be saved
///
/// # Returns
/// * `Result<ConversionResult, String>` - Success with conversion stats or error message
#[tauri::command]
async fn convert_files(
    window: tauri::Window,
    paths: Vec<String>,
    output_dir: String,
) -> Result<ConversionResult, String> {
    let converter = Converter::default();

    let base_path = std::path::Path::new(&output_dir).join("converted-files");
    let output_dir = get_unique_dir(base_path).map_err(|e| e.to_string())?;

    std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    let mut all_files = Vec::new();

    for path_str in paths {
        let path = std::path::Path::new(&path_str);
        let parent_dir = path.parent().unwrap_or(std::path::Path::new("."));

        let file = std::fs::read_dir(parent_dir)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .find(|entry| entry.path() == path)
            .ok_or_else(|| format!("error: file not found at {:?}", path))?;

        all_files.push(file);
    }

    converter
        .process_files(all_files, &output_dir, window)
        .map_err(|e| e.to_string())
}

/// Creates a unique dir path by appending a number if the base path already exists
///
/// # Examples
/// if in a given path, `foo` directory exists, `(1)` or any increments will be appended to avoid
/// overwrites in particular.
///
/// # Arguments
/// * `base_path` - Initial directory path
///
/// # Returns
/// * `Result<PathBuf, io::Error>` - Unique directory path or IO error
fn get_unique_dir(base_path: std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    // return original path if it doesn't exist yet
    if !base_path.exists() {
        return Ok(base_path);
    }

    let base_name = base_path.to_string_lossy();
    // append until a unique path is found
    let mut i = 1;
    loop {
        let new_path = std::path::PathBuf::from(format!("{}({})", base_name, i));
        if !new_path.exists() {
            return Ok(new_path);
        }
        i += 1;
    }
}

/// Initializes and runs VR8 Converter Tauri application
/// Sets up plugins, event handlers and the main window
/// Exits the application when the main window is closed
/// ProcMacro (ln89) might throw a warning on some IDE's, it's safe to ignore
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

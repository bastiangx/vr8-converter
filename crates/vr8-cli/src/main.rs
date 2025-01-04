use clap::Parser;
use std::env;
use std::path::{Path, PathBuf};
use tokio::fs;
use vr8_core::convert_files;

/// CLI main tool for conversion
#[derive(Parser)]
#[command(
    author = "bastiangx",
    version,
    long_about = None,
    about = "Convert VR8 audio files to WAV format (mono, 44kHz, 16-Bit)")]
struct Cli {
    /// Input VR8 files to convert
    #[arg(required = true, value_name = "FILES")]
    files: Vec<PathBuf>,

    // optional output (defaults to cwd)
    #[arg(
        short,
        long,
        value_name = "DIR",
        help = "Directory where the WAV files will be saved (default: cwd)"
    )]
    output_dir: Option<PathBuf>,
}

/// Format milliseconds into readable duration
fn format_duration(ms: u128) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let minutes = ms / 60000;
        let seconds = (ms % 60000) / 1000;
        format!("{}m {}s", minutes, seconds)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // if no dir specified, use cwd
    let output_dir = cli
        .output_dir
        .clone()
        .unwrap_or_else(|| env::current_dir().unwrap());

    fs::create_dir_all(&output_dir).await?;

    // cwd
    let current_dir = env::current_dir()?;

    let paths: Vec<String> = cli
        .files
        .iter()
        .map(|p| {
            if p.is_absolute() {
                p.clone()
            } else {
                current_dir.join(p)
            }
        })
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let result = convert_files(None, paths, output_dir.to_string_lossy().to_string()).await?;

    println!(
        "✨ Converted {} {} in {}",
        result.files_converted,
        if result.files_converted == 1 {
            "file"
        } else {
            "files"
        },
        format_duration(result.duration_ms)
    );

    let display_path = if cli.output_dir.is_some() {
        Path::new(&result.output_dir)
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| result.output_dir.as_str().into())
    } else {
        result.output_dir.as_str().into()
    };
    println!("📁 Output: {}", display_path);

    Ok(())
}

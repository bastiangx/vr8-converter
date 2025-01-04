pub use crate::error::ConverterError;

use hound::{SampleFormat, WavSpec, WavWriter};
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::{Emitter, Window};

/// Represents the results of a batch conversion operation
#[derive(Serialize)]
pub struct ConversionResult {
    pub files_converted: usize,
    pub duration_ms: u128,
    pub output_dir: String,
}

/// Handles the conversion of VR8 files to WAV format
pub struct Converter {
    wav_spec: WavSpec,
}

impl Default for Converter {
    fn default() -> Self {
        Self {
            wav_spec: WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            },
        }
    }
}

/// Creates a unique directory path by appending a number if the path already exists
pub fn get_unique_dir(base_path: PathBuf) -> Result<PathBuf, std::io::Error> {
    if !base_path.exists() {
        return Ok(base_path);
    }
    let base_name = base_path.to_string_lossy();
    let mut i = 1;
    loop {
        let new_path = PathBuf::from(format!("{}({})", base_name, i));
        if !new_path.exists() {
            return Ok(new_path);
        }
        i += 1;
    }
}

/// Main conversion func that processes multiple VR8 files in parallel using Rayon
///
/// # Arguments
/// * `window` - Optional Tauri window for progress updates
/// * `paths` - Vector of file paths to convert
/// * `output_dir` - Directory to save converted files
pub async fn convert_files(
    window: Option<Window>,
    paths: Vec<String>,
    output_dir: String,
) -> Result<ConversionResult, String> {
    println!("-------------------------");
    println!("Converting to WAV (Mono, 44.1kHz, 16-bit PCM)");
    println!("-------------------------");
    let converter = Converter::default();
    let output_dir = Path::new(&output_dir);

    fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

    let mut all_files = Vec::new();
    for path_str in paths {
        let path = Path::new(&path_str);
        let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));

        let file = fs::read_dir(parent_dir)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .find(|entry| entry.path() == path)
            .ok_or_else(|| format!("error: file not found at {:?}", path))?;

        all_files.push(file);
    }

    let start = std::time::Instant::now();
    let total_files = all_files.len() as u64;
    let processed_files = std::sync::atomic::AtomicUsize::new(0);
    let pb = ProgressBar::new(total_files);

    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{bar:25} {pos}/{len}")
            .unwrap()
            .progress_chars("=-"),
    );

    let results: Result<Vec<()>, ConverterError> = all_files
        .par_iter()
        .map(|entry| {
            let result = converter.convert_file(&entry.path(), output_dir);
            let current = processed_files.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let progress = ((current + 1) as f64 / total_files as f64 * 100.0) as u32;

            if let Some(w) = &window {
                w.emit("conversion_progress", progress).unwrap();
            }
            pb.inc(1);
            result
        })
        .collect();

    results.map_err(|e| e.to_string())?;

    Ok(ConversionResult {
        files_converted: all_files.len(),
        duration_ms: start.elapsed().as_millis(),
        output_dir: output_dir.to_string_lossy().into_owned(),
    })
}

impl Converter {
    fn convert_file(&self, input_path: &Path, output_dir: &Path) -> Result<(), ConverterError> {
        let file_size = fs::metadata(input_path)?.len();
        let chunk_threshold = 30 * 1024 * 1024; // 30mb

        let input_filename = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| ConverterError::InvalidData("Invalid input filename".to_string()))?;

        let output_path = output_dir.join(format!("{}.wav", input_filename));
        let file = fs::File::open(input_path)?;
        let mut writer = WavWriter::create(&output_path, self.wav_spec)?;

        // for larger files, to avoid mem crash
        if file_size > chunk_threshold {
            let mut reader = std::io::BufReader::with_capacity(64 * 1024, file);
            let mut buffer = [0u8; 64 * 1024];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                for chunk in buffer[..bytes_read].chunks(2) {
                    if chunk.len() == 2 {
                        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                        writer.write_sample(sample)?;
                    }
                }
            }
        } else {
            let mut reader = std::io::BufReader::new(file);
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;
            for chunk in data.chunks(2) {
                if chunk.len() == 2 {
                    let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                    writer.write_sample(sample)?;
                }
            }
        }

        writer.finalize()?;
        Ok(())
    }
}

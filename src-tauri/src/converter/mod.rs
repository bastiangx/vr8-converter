mod error;
pub use error::ConverterError;

use hound::{SampleFormat, WavSpec, WavWriter};
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;
use tauri::Emitter as TauriManager;

/// Holds the result of a batch conversion operation
#[derive(Serialize)]
pub struct ConversionResult {
    /// Number of files successfully converted
    files_converted: usize,
    /// Total time taken in milliseconds
    duration_ms: u128,
    /// Output directory where the files were saved
    output_dir: String,
}

/// Main converter struct handling VR8 to WAV conversion
pub struct Converter {
    /// WAV file format specification
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

impl Converter {
    /// Converts a single VR8 file to WAV format
    ///
    /// Uses chunked reading for files larger than 30MB to prevent memory spike/panic.
    ///
    /// # Arguments
    /// * `input_path` - Path to the VR8 file
    /// * `output_dir` - Directory where the WAV file will be saved
    fn convert_file(&self, input_path: &Path, output_dir: &Path) -> Result<(), ConverterError> {
        let file_size = fs::metadata(input_path)?.len();
        let chunk_threshold = 30 * 1024 * 1024; // 30mb

        // extract input filename w/o extension
        let input_filename = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| ConverterError::InvalidData("Invalid input filename".to_string()))?;
        let output_path = output_dir.join(format!("{}.wav", input_filename));

        let file = fs::File::open(input_path)?;
        let mut writer = WavWriter::create(&output_path, self.wav_spec)?;

        if file_size > chunk_threshold {
            // chunk reading in 64kb calls
            // READER/BUFFER SIZE MUST MATCH AND BE A MULTIPLE OF 2
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
            // default memory reading
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

    /// Process multiple VR8 files in parallel
    ///
    /// # Arguments
    /// * `files` - Vector of files to convert
    /// * `output_dir` - Output directory for converted files
    /// * `window` - Tauri window for progress updates
    ///
    /// # Returns
    /// * `Result<ConversionResult, ConverterError>` - Conversion stats or error
    pub fn process_files(
        &self,
        files: Vec<fs::DirEntry>,
        output_dir: &Path,
        window: tauri::Window,
    ) -> Result<ConversionResult, ConverterError> {
        let start = std::time::Instant::now();
        let total_files = files.len() as u64;

        // track progress purely for UI
        let processed_files = std::sync::atomic::AtomicUsize::new(0);
        let pb = ProgressBar::new(total_files);

        // in parallel using rayon (rayon is the goat)
        let results: Result<Vec<()>, ConverterError> = files
            .par_iter()
            .map(|entry| {
                let result = self.convert_file(&entry.path(), output_dir);
                let current = processed_files.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let progress = ((current + 1) as f64 / total_files as f64 * 100.0) as u32;
                window.emit("conversion_progress", progress).unwrap();
                pb.inc(1);
                result
            })
            .collect();

        results?;
        pb.finish_with_message("all files converted!");

        Ok(ConversionResult {
            files_converted: files.len(),
            duration_ms: start.elapsed().as_millis(),
            output_dir: output_dir.to_string_lossy().into_owned(),
        })
    }
}

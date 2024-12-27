mod error;
pub use error::ConverterError;

use hound::{SampleFormat, WavSpec, WavWriter};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::Serialize;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Serialize)]
pub struct ConversionResult {
    files_converted: usize,
    duration_ms: u128,
    output_dir: String,
}

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

impl Converter {
    fn convert_file(&self, input_path: &Path, output_dir: &Path) -> Result<(), ConverterError> {
        let input_filename = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| ConverterError::InvalidData("Invalid input filename".to_string()))?;

        let output_path = output_dir.join(format!("{}.wav", input_filename));

        let file = fs::File::open(input_path)?;
        let mut reader = BufReader::with_capacity(1024 * 1024, file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        let mut writer = WavWriter::create(&output_path, self.wav_spec)?;

        for chunk in data.chunks(2) {
            if chunk.len() == 2 {
                let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                writer.write_sample(sample)?;
            }
        }

        writer.finalize()?;
        Ok(())
    }

    pub fn process_files(
        &self,
        files: Vec<fs::DirEntry>,
        output_dir: &Path,
    ) -> Result<ConversionResult, ConverterError> {
        let start = std::time::Instant::now();

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        let results: Result<Vec<()>, ConverterError> = files
            .par_iter()
            .map(|entry| {
                let result = self.convert_file(&entry.path(), output_dir);
                pb.inc(1);
                result
            })
            .collect();

        results?;

        pb.finish_with_message("✨ All files converted!");

        Ok(ConversionResult {
            files_converted: files.len(),
            duration_ms: start.elapsed().as_millis(),
            output_dir: output_dir.to_string_lossy().into_owned(),
        })
    }
}

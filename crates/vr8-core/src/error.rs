use std::fmt;

/// Custom error type for the converter module
#[derive(Debug)]
pub enum ConverterError {
    /// Wrapper for standard IO errors
    IoError(std::io::Error),
    /// Wrapper for 'hound' WAV encoding/decoding errors
    WavError(hound::Error),
    /// Custom error for invalid data with a message
    InvalidData(String),
}

/// Implement Display trait for custom error type
impl fmt::Display for ConverterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConverterError::IoError(e) => write!(f, "IO error: {}", e),
            ConverterError::WavError(e) => write!(f, "WAV error: {}", e),
            ConverterError::InvalidData(s) => write!(f, "Invalid data: {}", s),
        }
    }
}

/// Implement Automatic conversion from std::io::Error to custom error type
impl From<std::io::Error> for ConverterError {
    fn from(err: std::io::Error) -> Self {
        ConverterError::IoError(err)
    }
}

/// Implement Automatic conversion from hound::Error to custom error type
impl From<hound::Error> for ConverterError {
    fn from(err: hound::Error) -> Self {
        ConverterError::WavError(err)
    }
}

impl std::error::Error for ConverterError {}

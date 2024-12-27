use std::fmt;

#[derive(Debug)]
pub enum ConverterError {
    IoError(std::io::Error),
    WavError(hound::Error),
    InvalidData(String),
}

impl fmt::Display for ConverterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConverterError::IoError(e) => write!(f, "IO error: {}", e),
            ConverterError::WavError(e) => write!(f, "WAV error: {}", e),
            ConverterError::InvalidData(s) => write!(f, "Invalid data: {}", s),
        }
    }
}

impl From<std::io::Error> for ConverterError {
    fn from(err: std::io::Error) -> Self {
        ConverterError::IoError(err)
    }
}

impl From<hound::Error> for ConverterError {
    fn from(err: hound::Error) -> Self {
        ConverterError::WavError(err)
    }
}

impl std::error::Error for ConverterError {}

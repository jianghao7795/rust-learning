use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use stage04_text_stats::{TextStats, analyze};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "读取文件失败：{error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn analyze_file(path: impl AsRef<Path>) -> Result<TextStats, AppError> {
    let content = fs::read_to_string(path)?;
    Ok(analyze(&content))
}

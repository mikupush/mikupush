use std::error::Error;
use std::fs;
use std::path::PathBuf;
use log::warn;
use mimetype_detector::detect_file;
use regex::Regex;

#[derive(Debug)]
pub enum MimeTypeDetectError {
    DetectError { message: String },
    PathError { message: String },
    IOError { message: String },
}

impl std::fmt::Display for MimeTypeDetectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MimeTypeDetectError::DetectError { message } => write!(f, "failed to detect mime type: {}", message),
            MimeTypeDetectError::PathError { message } => write!(f, "path is not valid unicode: {}", message),
            MimeTypeDetectError::IOError { message } => write!(f, "failed to read file: {}", message),
        }
    }
}

impl Error for MimeTypeDetectError {}

pub fn detect_mime_type(path: PathBuf) -> Result<String, MimeTypeDetectError> {
    if is_svg_image(&path)? {
        return Ok("image/svg+xml".to_string());
    }

    let path_str = path.to_str().ok_or_else(|| {
        MimeTypeDetectError::PathError { message: "path is not valid unicode".to_string() }
    })?;

    let mime_type = detect_file(&path_str).map_err(|err| {
        warn!("failed to detect mime type for path {}: {}", path.display(), err);
        MimeTypeDetectError::DetectError { message: err.to_string() }
    })?;

    Ok(mime_type.mime().to_string())
}

fn is_svg_image(path: &PathBuf) -> Result<bool, MimeTypeDetectError> {
    let bytes = fs::read(path).map_err(|err| {
        MimeTypeDetectError::IOError { message: err.to_string() }
    })?;

    let regex = Regex::new(r"^<svg .*").unwrap();
    Ok(regex.is_match(&String::from_utf8_lossy(&bytes)))
}
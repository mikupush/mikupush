use std::error::Error;
use std::path::PathBuf;
use mimetype_detector::{detect, detect_file};
use base64::Engine;
use base64::engine::general_purpose;
use log::warn;
use crate::mime_type::detect_mime_type;

#[derive(Debug)]
pub enum ImageEncodeError {
    ReadError { message: String },
    NotImageError { message: String },
    DetectTypeError { message: String }
}

impl std::fmt::Display for ImageEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageEncodeError::ReadError { message } => write!(f, "read error: {}", message),
            ImageEncodeError::NotImageError { message } => write!(f, "not image error: {}", message),
            ImageEncodeError::DetectTypeError { message } => write!(f, "detect type error: {}", message)
        }
    }
}

impl Error for ImageEncodeError {}

pub fn encode_image_base64(path: PathBuf) -> Result<String, ImageEncodeError> {
    let mime_type = detect_mime_type(path.clone()).map_err(|err| {
        warn!("failed to detect mime type: {}", err);
        ImageEncodeError::DetectTypeError { message: err.to_string() }
    })?;

    if !mime_type.starts_with("image/") {
        return Err(ImageEncodeError::NotImageError {
            message: format!("expected image got: {}", mime_type)
        });
    }

    let bytes = std::fs::read(path.clone()).map_err(|err| {
        ImageEncodeError::ReadError { message: err.to_string() }
    })?;

    let base64 = general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime_type, base64))
}


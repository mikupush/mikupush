// Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
// Copyright (C) 2025  Miku Push! Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use log::{debug, warn};
use mime_guess::from_path;
use mimetype_detector::detect_file;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug)]
pub enum MimeTypeDetectError {
    DetectError(String),
    PathError(String),
    IOError(String),
}

impl std::fmt::Display for MimeTypeDetectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MimeTypeDetectError::DetectError(message) => {
                write!(f, "failed to detect mime type: {}", message)
            }
            MimeTypeDetectError::PathError(message) => {
                write!(f, "path is not valid unicode: {}", message)
            }
            MimeTypeDetectError::IOError(message) => {
                write!(f, "failed to read file: {}", message)
            }
        }
    }
}

impl Error for MimeTypeDetectError {}

pub fn detect_mime_type_by_content(path: PathBuf) -> Result<String, MimeTypeDetectError> {
    let start = Instant::now();
    if is_svg_image(&path)? {
        return Ok("image/svg+xml".to_string());
    }

    let path_str = path
        .to_str()
        .ok_or_else(|| MimeTypeDetectError::PathError("path is not valid unicode".to_string()))?;

    let mime_type = detect_file(&path_str).map_err(|err| {
        warn!(
            "failed to detect mime type for path {}: {}",
            path.display(),
            err
        );
        MimeTypeDetectError::DetectError(err.to_string())
    })?;

    debug!("resolved mime-type: {:?} in {:?}", path_str, start.elapsed());
    Ok(mime_type.mime().to_string())
}

pub fn detect_mime_type_by_extension(path: PathBuf) -> Result<String, MimeTypeDetectError> {
    let start = Instant::now();
    let mime_type = from_path(&path)
        .first()
        .map(|mime_type| mime_type.essence_str().to_string())
        .ok_or_else(|| MimeTypeDetectError::DetectError(format!(
            "failed to detect mime type by contents or extension for path {}",
            path.display(),
        )));

    debug!("resolved from extension mime-type: {:?} in {:?}", mime_type, start.elapsed());
    mime_type
}

fn is_svg_image(path: &PathBuf) -> Result<bool, MimeTypeDetectError> {
    let bytes = fs::read(path).map_err(|err| MimeTypeDetectError::IOError(err.to_string()))?;

    let regex = Regex::new(r"^<svg .*").unwrap();
    Ok(regex.is_match(&String::from_utf8_lossy(&bytes)))
}

const MAX_FILE_SIZE: u64 = 512 * 1024 * 1024; // 512 MB
const FALLBACK_MIME_TYPE: &str = "application/octet-stream";

pub fn detect_mime_type(path: PathBuf) -> Result<String, MimeTypeDetectError> {
    let stats = match fs::metadata(&path) {
        Ok(stats) => stats,
        Err(err) => return Err(MimeTypeDetectError::IOError(err.to_string())),
    };

    let size = stats.len();
    if size > MAX_FILE_SIZE {
        debug!("file {} is too large, detecting mime type by its extension", path.display());
        let mime_type = detect_mime_type_by_extension(path.to_path_buf())
            .unwrap_or(FALLBACK_MIME_TYPE.to_string());

        return Ok(mime_type)
    }

    let mime_type = match detect_mime_type_by_content(path.to_path_buf()) {
        Ok(mime_type) => mime_type.to_string(),
        Err(_) => detect_mime_type_by_extension(path.to_path_buf())
            .unwrap_or(FALLBACK_MIME_TYPE.to_string()),
    };

    Ok(mime_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_mime_type_by_extension_should_fallback_to_file_extension() {
        let mime_type = detect_mime_type_by_extension(PathBuf::from("missing-file.pdf")).unwrap();

        assert_eq!("application/pdf", mime_type);
    }

    #[test]
    fn detect_mime_type_by_extension_should_return_error_when_extension_is_unknown() {
        let result = detect_mime_type_by_extension(PathBuf::from("missing-file"));

        assert!(result.is_err());
    }
}

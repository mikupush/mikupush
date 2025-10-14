// Copyright 2025 Miku Push! Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::response::ErrorResponse;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait ClientError: Debug + Display + Error {
    fn code(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileUploadError {
    Exists { message: String },
    NotExists { message: String },
    MaxFileSizeExceeded { message: String },
    NotCompleted { message: String },
    UnknownMimeType,
    Canceled,
    InternalServerError { message: String },
    ClientError { message: String },
}

impl From<ErrorResponse> for FileUploadError {
    fn from(value: ErrorResponse) -> Self {
        match value.code.as_str() {
            "Exists" => Self::Exists { message: value.message },
            "NotExists" => Self::NotExists { message: value.message },
            "MaxFileSizeExceeded" => Self::MaxFileSizeExceeded { message: value.message },
            _ => Self::InternalServerError { message: value.message },
        }
    }
}

impl Display for FileUploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exists { message } => write!(f, "{}", message),
            Self::NotExists { message } => write!(f, "{}", message),
            Self::MaxFileSizeExceeded { message } => write!(f, "{}", message),
            Self::NotCompleted { message } => write!(f, "{}", message),
            Self::UnknownMimeType => write!(f, "unknown mime type for the provided file to upload"),
            Self::Canceled => write!(f, "file upload has been canceled"),
            Self::InternalServerError { message } => write!(f, "{}", message),
            Self::ClientError { message } => write!(f, "{}", message),
        }
    }
}

impl Error for FileUploadError {}

impl ClientError for FileUploadError {
    fn code(&self) -> String {
        match self {
            Self::Exists { .. } => FILE_UPLOAD_ERROR_EXISTS.to_string(),
            Self::NotExists { .. } => FILE_UPLOAD_ERROR_NOT_EXISTS.to_string(),
            Self::MaxFileSizeExceeded { .. } => FILE_UPLOAD_ERROR_MAX_FILE_SIZE_EXCEEDED.to_string(),
            Self::NotCompleted { .. } => FILE_UPLOAD_ERROR_NOT_COMPLETED.to_string(),
            Self::UnknownMimeType => FILE_UPLOAD_ERROR_UNKNOWN_MIME_TYPE.to_string(),
            Self::Canceled => FILE_UPLOAD_ERROR_CANCELED.to_string(),
            Self::InternalServerError { .. } => FILE_UPLOAD_ERROR_INTERNAL_SERVER_ERROR.to_string(),
            Self::ClientError { .. } => FILE_UPLOAD_ERROR_CLIENT_ERROR.to_string(),
        }
    }
}

pub const FILE_UPLOAD_ERROR_EXISTS: &str = "exists";
pub const FILE_UPLOAD_ERROR_NOT_EXISTS: &str = "not_exists";
pub const FILE_UPLOAD_ERROR_MAX_FILE_SIZE_EXCEEDED: &str = "max_file_size_exceeded";
pub const FILE_UPLOAD_ERROR_NOT_COMPLETED: &str = "not_completed";
pub const FILE_UPLOAD_ERROR_UNKNOWN_MIME_TYPE: &str = "unknown_mime_type";
pub const FILE_UPLOAD_ERROR_CANCELED: &str = "canceled";
pub const FILE_UPLOAD_ERROR_INTERNAL_SERVER_ERROR: &str = "internal_server_error";
pub const FILE_UPLOAD_ERROR_CLIENT_ERROR: &str = "client_error";


#[derive(Debug, Clone, PartialEq)]
pub enum FileDeleteError {
    NotExists { message: String },
    InternalServerError { message: String },
    ClientError { message: String },
}

impl From<ErrorResponse> for FileDeleteError {
    fn from(value: ErrorResponse) -> Self {
        match value.code.as_str() {
            "NotExists" => Self::NotExists { message: value.message },
            _ => Self::InternalServerError { message: value.message },
        }
    }
}

impl Display for FileDeleteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExists { message } => write!(f, "{}", message),
            Self::InternalServerError { message } => write!(f, "{}", message),
            Self::ClientError { message } => write!(f, "{}", message),
        }
    }
}

impl Error for FileDeleteError {}

impl ClientError for FileDeleteError {
    fn code(&self) -> String {
        match self {
            Self::NotExists { .. } => FILE_DELETE_ERROR_NOT_EXISTS.to_string(),
            Self::InternalServerError { .. } => FILE_DELETE_ERROR_INTERNAL_SERVER_ERROR.to_string(),
            Self::ClientError { .. } => FILE_DELETE_ERROR_CLIENT_ERROR.to_string(),
        }
    }
}

pub const FILE_DELETE_ERROR_NOT_EXISTS: &str = "not_exists";
pub const FILE_DELETE_ERROR_INTERNAL_SERVER_ERROR: &str = "internal_server_error";
pub const FILE_DELETE_ERROR_CLIENT_ERROR: &str = "client_error";

#[derive(Debug, Clone, PartialEq)]
pub enum FileInfoError {
    NotExists { message: String },
    InternalServerError { message: String },
    ClientError { message: String },
}

impl From<ErrorResponse> for FileInfoError {
    fn from(value: ErrorResponse) -> Self {
        match value.code.as_str() {
            "NotExists" => Self::NotExists { message: value.message },
            _ => Self::InternalServerError { message: value.message },
        }
    }
}

impl Display for FileInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExists { message } => write!(f, "{}", message),
            Self::InternalServerError { message } => write!(f, "{}", message),
            Self::ClientError { message } => write!(f, "{}", message),
        }
    }
}

impl Error for FileInfoError {}

impl ClientError for FileInfoError {
    fn code(&self) -> String {
        match self {
            Self::NotExists { .. } => FILE_INFO_ERROR_NOT_EXISTS.to_string(),
            Self::InternalServerError { .. } => FILE_INFO_ERROR_INTERNAL_SERVER_ERROR.to_string(),
            Self::ClientError { .. } => FILE_INFO_ERROR_CLIENT_ERROR.to_string(),
        }
    }
}

pub const FILE_INFO_ERROR_NOT_EXISTS: &str = "not_exists";
pub const FILE_INFO_ERROR_INTERNAL_SERVER_ERROR: &str = "internal_server_error";
pub const FILE_INFO_ERROR_CLIENT_ERROR: &str = "client_error";

#[derive(Debug, Clone, PartialEq)]
pub struct HealthCheckError {
    pub(crate) message: String
}

impl Display for HealthCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HealthCheckError {}

impl ClientError for HealthCheckError {
    fn code(&self) -> String {
        HEALTH_CHECK_ERROR_HEALTH_CHECK_ERROR.to_string()
    }
}

pub const HEALTH_CHECK_ERROR_HEALTH_CHECK_ERROR: &str = "health_check_error";
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
            Self::Exists { .. } => "exists".to_string(),
            Self::NotExists { .. } => "not_exists".to_string(),
            Self::MaxFileSizeExceeded { .. } => "max_file_size_exceeded".to_string(),
            Self::NotCompleted { .. } => "not_completed".to_string(),
            Self::UnknownMimeType => "unknown_mime_type".to_string(),
            Self::Canceled => "canceled".to_string(),
            Self::InternalServerError { .. } => "internal_server_error".to_string(),
            Self::ClientError { .. } => "client_error".to_string(),
        }
    }
}

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
            Self::NotExists { .. } => "not_exists".to_string(),
            Self::InternalServerError { .. } => "internal_server_error".to_string(),
            Self::ClientError { .. } => "client_error".to_string(),
        }
    }
}

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
        "health_check_error".to_string()
    }
}
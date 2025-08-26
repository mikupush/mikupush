use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use reqwest::Response;
use tokio::task::JoinError;

#[derive(Debug, Clone)]
pub struct ClientError {
    pub message: String,
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ClientError {}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        Self { message: error.to_string() }
    }
}

#[derive(Debug)]
pub struct ServerResponseError {
    status: u16,
    reason: String,
}

impl ServerResponseError {
    pub fn from_response(response: Response) -> Self {
        Self {
            status: response.status().into(),
            reason: response.status().canonical_reason()
                .or_else(|| Some(""))
                .unwrap()
                .to_string()
        }
    }
}

impl Display for ServerResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "server respond with status: {}: {}", self.status, self.reason)
    }
}

impl Error for ServerResponseError {}

impl From<ServerResponseError> for ClientError {
    fn from(error: ServerResponseError) -> Self {
        Self { message: error.to_string() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UploadError {
    IO { message: String },
    Client { message: String },
    Canceled,
    JoinError { message: String },
    Server { message: String },
}

impl From<ServerResponseError> for UploadError {
    fn from(error: ServerResponseError) -> Self {
        UploadError::Server {
            message: format!(
                "server respond error with status code: {}: {}",
                error.status, error.reason
            ),
        }
    }
}

impl From<JoinError> for UploadError {
    fn from(m: JoinError) -> Self {
        if m.is_cancelled() {
            return UploadError::Canceled;
        }

        UploadError::JoinError {
            message: format!("upload task join error: {}", m),
        }
    }
}

impl From<reqwest::Error> for UploadError {
    fn from(m: reqwest::Error) -> Self {
        UploadError::Client {
            message: format!("error during the http request: {}", m),
        }
    }
}

impl From<ClientError> for UploadError {
    fn from(m: ClientError) -> Self {
        UploadError::Client { message: format!("{}", m), }
    }
}

impl From<std::io::Error> for UploadError {
    fn from(m: std::io::Error) -> Self {
        UploadError::IO {
            message: format!("error during I/O operation: {}", m),
        }
    }
}

impl Display for UploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            UploadError::IO { message } => message.clone(),
            UploadError::Client { message } => message.clone(),
            UploadError::Canceled => "upload was canceled".to_string(),
            UploadError::JoinError { message } => message.clone(),
            UploadError::Server { message } => message.clone(),
        };

        write!(f, "{}",message)
    }
}

impl Error for UploadError {}
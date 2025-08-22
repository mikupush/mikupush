use crate::models::{Upload, UploadRequest};
use crate::GenericResult;
use log::debug;
use reqwest::Response;
use serde::Serialize;
use serde_json::json;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::select;
use tokio::sync::watch;
use tokio::task::{JoinError, JoinHandle};
use tokio_stream::StreamExt;
use tokio_util::{io::ReaderStream, sync::CancellationToken};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub base_url: String,
    pub name: String,
}

impl Default for Server {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        let base_url = "http://localhost:8080".to_string();

        #[cfg(not(debug_assertions))]
        let base_url = "https://mikupush.io".to_string();

        Self {
            base_url,
            name: "mikupush.io".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEvent {
    pub upload_id: Uuid,
    pub progress: f32,
    pub total_size: u64,
    pub uploaded_bytes: u64,
    pub rate_bytes: u64,
    last_uploaded_bytes: u64,
}

impl ProgressEvent {
    pub fn new(upload_id: Uuid, total_size: u64) -> Self {
        Self {
            upload_id,
            progress: 0.0,
            total_size,
            uploaded_bytes: 0,
            rate_bytes: 0,
            last_uploaded_bytes: 0,
        }
    }

    pub fn update(&mut self, uploaded_bytes: u64) -> Self {
        self.uploaded_bytes = uploaded_bytes;
        self.progress = self.calculate_progress(uploaded_bytes);
        self.rate_bytes = self.calculate_rate(uploaded_bytes);
        self.clone()
    }

    fn calculate_progress(&self, uploaded_bytes: u64) -> f32 {
        if self.total_size > 0 {
            uploaded_bytes as f32 / self.total_size as f32
        } else {
            0.0
        }
    }

    fn calculate_rate(&mut self, uploaded_bytes: u64) -> u64 {
        let rate_bytes = uploaded_bytes.saturating_sub(self.last_uploaded_bytes);
        self.last_uploaded_bytes = uploaded_bytes;
        rate_bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UploadError {
    FileSystem { message: String },
    Http { message: String },
    Canceled,
    JoinError { message: String },
    ServerError { message: String },
}

impl UploadError {
    pub fn from_response(response: Response) -> Self {
        let status = response.status();
        let reason = status.canonical_reason().unwrap_or("");

        UploadError::ServerError {
            message: format!(
                "server respond error with status code: {}: {}",
                status, reason
            ),
        }
    }

    pub fn message(&self) -> String {
        match self {
            UploadError::FileSystem { message } => message.clone(),
            UploadError::Http { message } => message.clone(),
            UploadError::Canceled => "upload was canceled".to_string(),
            UploadError::JoinError { message } => message.clone(),
            UploadError::ServerError { message } => message.clone(),
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
        UploadError::Http {
            message: format!("error during the http request: {}", m),
        }
    }
}

impl From<std::io::Error> for UploadError {
    fn from(m: std::io::Error) -> Self {
        UploadError::FileSystem {
            message: format!("error during file operation: {}", m),
        }
    }
}

impl Display for UploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub struct UploadTask {
    cancellation_token: CancellationToken,
    handle: JoinHandle<Result<(), UploadError>>,
    progress_receiver: watch::Receiver<ProgressEvent>,
}

impl UploadTask {
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
        if !self.handle.is_finished() {
            self.handle.abort();
        }
    }

    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    pub async fn wait(self) -> Result<(), UploadError> {
        self.handle
            .await
            .unwrap_or_else(|join_err| Err(join_err.into()))
    }

    pub fn progress(&self) -> watch::Receiver<ProgressEvent> {
        self.progress_receiver.clone()
    }
}

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(server: Server) -> Self {
        debug!("using server client with base url: {}", server.base_url);
        Self {
            base_url: server.base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn create(&self, upload: &Upload) -> GenericResult<()> {
        let data = json!({
            "uuid": upload.id,
            "name": upload.name,
            "mime_type": upload.mime_type,
            "size": upload.size
        });

        let url = format!("{}/api/file", self.base_url);
        let response = self.client.post(&url).json(&data).send().await?;

        if response.status() != 200 {
            return Err(format!(
                "upload create request failed with status {}",
                response.status()
            )
            .into());
        }

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> GenericResult<()> {
        let url = format!("{}/api/file/{}", self.base_url, id);
        let response = self.client.delete(&url).send().await?;

        if response.status() != 200 {
            return Err(format!(
                "upload delete request failed with status {}",
                response.status()
            )
            .into());
        }

        Ok(())
    }

    pub async fn upload(&self, request: &UploadRequest) -> Result<UploadTask, UploadError> {
        if request.upload.mime_type.is_empty() {
            return Err(UploadError::FileSystem {
                message: "unknown file type".to_string(),
            });
        }

        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let request = request.clone();
        let cancellation_token = CancellationToken::new();
        let handle_cancellation_token = cancellation_token.clone();
        let (progress_sender, progress_receiver) =
            watch::channel(ProgressEvent::new(request.upload.id, request.upload.size));

        let handle: JoinHandle<Result<(), UploadError>> = tokio::spawn(async move {
            let file_path = Path::new(&request.upload.path);
            let file = File::open(file_path).await?;
            let total_size = file.metadata().await?.len();
            let mut reader_stream = ReaderStream::new(file);
            let stream_cancellation_token = handle_cancellation_token.clone();

            let stream = async_stream::stream! {
                let mut progress_event = ProgressEvent::new(request.upload.id, total_size);
                let mut uploaded_bytes: u64 = 0;
                let mut last_measured_rate = Instant::now();

                while let Some(chunk) = reader_stream.next().await {
                    if stream_cancellation_token.is_cancelled() {
                        break;
                    }

                    if let Ok(chunk) = &chunk {
                        uploaded_bytes = min(uploaded_bytes + (chunk.len() as u64), total_size);
                        let elapsed = last_measured_rate.elapsed();

                        if elapsed >= Duration::from_secs(1) {
                            let updated_progress = progress_event.update(uploaded_bytes);
                            let _ = progress_sender.send(updated_progress);
                            last_measured_rate = Instant::now();
                        }

                        #[cfg(test)]
                        {
                            tokio::time::sleep(Duration::from_millis(5)).await;
                        }
                    }

                    yield chunk;
                }

                let updated_progress = progress_event.update(uploaded_bytes);
                let _ = progress_sender.send(updated_progress);
            };

            let body = reqwest::Body::wrap_stream(stream);
            let url = format!("{}/api/file/{}/upload", base_url, request.upload.id);
            let send_future = client
                .post(&url)
                .header("Content-Type", &request.upload.mime_type)
                .header("Content-Length", total_size.to_string())
                .body(body)
                .send();

            let response = select! {
                res = send_future => res?,
                _ = handle_cancellation_token.cancelled() => {
                    return Err(UploadError::Canceled);
                }
            };

            if response.status() != 200 {
                return Err(UploadError::from_response(response));
            }

            Ok(())
        });

        Ok(UploadTask {
            cancellation_token,
            handle,
            progress_receiver,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    impl Client {
        pub fn test() -> Self {
            Self {
                base_url: "http://localhost:8080".to_string(),
                client: reqwest::Client::new(),
            }
        }
    }

    #[tokio::test]
    async fn server_client_create_should_create_file() {
        let client = Client::test();
        let upload = UploadRequest::test().upload;
        let result = client.create(&upload).await;
        println!("create file result: {:?}", result);

        assert_eq!(true, result.is_ok());
    }

    #[tokio::test]
    async fn server_client_delete_should_delete_file() {
        let client = Client::test();
        let upload = UploadRequest::test().upload;
        let id = upload.id.clone();

        client.create(&upload).await.unwrap();
        let result = client.delete(id).await;
        println!("delete file result: {:?}", result);

        assert_eq!(true, result.is_ok());
    }

    #[tokio::test]
    async fn server_client_delete_should_not_delete_not_existing_file() {
        let client = Client::test();
        let id = Uuid::new_v4();
        let result = client.delete(id).await;
        println!("delete not existing file result: {:?}", result);

        assert_eq!(true, result.is_ok());
    }

    #[tokio::test]
    async fn server_client_upload_should_upload_file() {
        let client = Client::test();
        let upload_request = UploadRequest::test();

        client.create(&upload_request.upload).await.unwrap();
        let task = client.upload(&upload_request).await.unwrap();
        let result = task.wait().await;
        println!("upload result {:?}", result);

        assert_eq!(true, result.is_ok())
    }

    #[tokio::test]
    async fn server_client_upload_should_cancel() {
        let client = Client::test();
        let upload_request = UploadRequest::test();

        let task = client.upload(&upload_request).await.unwrap();
        let mut receiver = task.progress();
        receiver.changed().await.unwrap();
        let progress = receiver.borrow().clone();
        println!("upload progress received: {:?}", progress);

        task.cancel();
        let result = task.wait().await;

        assert_eq!(UploadError::Canceled, result.unwrap_err())
    }
}

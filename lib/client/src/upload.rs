use crate::progress::ProgressTrack;
use crate::response::ErrorResponse;
use crate::FileUploadError;
use async_stream::__private::AsyncStream;
use bytes::Bytes;
use log::debug;
use mikupush_common::Upload;
use std::cmp::min;
use std::future::Future;
use std::io::Result as IoResult;
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::select;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tokio::sync::watch::Sender;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UploadStream {
    path: String,
    upload_id: String,
    total_size: u64,
    uploaded_bytes: u64,
    cancellation_token: CancellationToken,
    progress_sender: Sender<ProgressTrack>,
    progress: ProgressTrack,
    last_measured_rate: Instant,
}

impl UploadStream {
    pub async fn new (
        path: String,
        upload_id: Uuid,
        cancellation_token: CancellationToken,
        total_size: u64,
        progress: ProgressTrack,
        progress_sender: Sender<ProgressTrack>,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            path,
            upload_id: upload_id.to_string(),
            total_size,
            uploaded_bytes: 0,
            cancellation_token,
            progress_sender,
            progress,
            last_measured_rate: Instant::now(),
        })
    }

    async fn emit_progress(&mut self, chunk: &IoResult<Bytes>) {
        if let Ok(chunk) = chunk {
            self.uploaded_bytes = min(self.uploaded_bytes + (chunk.len() as u64), self.total_size);
            let elapsed = self.last_measured_rate.elapsed();

            if elapsed >= Duration::from_secs(1) {
                let updated_progress = self.progress.update(self.uploaded_bytes);
                let _ = self.progress_sender.send(updated_progress);
                self.last_measured_rate = Instant::now();
            }

            #[cfg(test)]
            {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        }
    }

    pub async fn to_async_stream(&self) -> Result<AsyncStream<IoResult<Bytes>, impl Future<Output=()>>, std::io::Error> {
        let mut upload_stream = self.clone();
        let mut reader = ReaderStream::new(File::open(self.path.clone()).await?);

        Ok(async_stream::stream! {
            while let Some(chunk) = reader.next().await {
                if upload_stream.cancellation_token.is_cancelled() {
                    debug!("upload canceled for: {}", upload_stream.upload_id);
                    break;
                }

                upload_stream.emit_progress(&chunk).await;
                yield chunk;
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct UploadTask {
    url: String,
    stream: UploadStream,
    pub progress_receiver: Receiver<ProgressTrack>,
    pub cancellation_token: CancellationToken,
    client: reqwest::Client,
    upload: Upload,
}

impl UploadTask {
    pub async fn new (
        url: String,
        upload: Upload,
        client: reqwest::Client,
    ) -> Result<Self, std::io::Error> {
        let progress = ProgressTrack::new(upload.id.clone(), upload.size);
        let cancellation_token = CancellationToken::new();

        let (
            progress_sender,
            progress_receiver
        ) = watch::channel(progress.clone());

        let stream = UploadStream::new(
            upload.path.clone(),
            upload.id.clone(),
            cancellation_token.clone(),
            upload.size,
            progress,
            progress_sender,
        ).await?;

        Ok(Self {
            url,
            stream,
            progress_receiver,
            cancellation_token,
            client,
            upload
        })
    }

    async fn perform_upload(&self) -> Result<(), FileUploadError> {
        let stream = self.stream.to_async_stream().await
            .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;
        let body = reqwest::Body::wrap_stream(stream);
        let send_future = self.client
            .post(&self.url)
            .header("Content-Type", &self.upload.mime_type)
            .header("Content-Length", self.upload.size)
            .body(body)
            .send();

        let response = select! {
                res = send_future => res.map_err(|err| {
                    FileUploadError::ClientError { message: err.to_string() }
                })?,
                _ = self.cancellation_token.cancelled() => {
                    return Err(FileUploadError::Canceled);
                }
            };

        if response.status() != 200 {
            let error_response = ErrorResponse::from_response(response).await
                .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;
            return Err(error_response.into());
        }

        Ok(())
    }

    pub fn start(&self) -> JoinHandle<Result<(), FileUploadError>> {
        let task = self.clone();

        tokio::spawn(async move {
            task.perform_upload().await
        })
    }
}
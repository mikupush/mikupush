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

use crate::progress::ProgressTrack;
use crate::response::ErrorResponse;
use crate::FileUploadError;
use bytes::Bytes;
use futures_core::Stream;
use log::debug;
use mikupush_common::Upload;
use std::cmp::min;
use std::io::Result as IoResult;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU64;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::select;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tokio::sync::watch::Sender;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug)]
pub struct UploadStream {
    upload_id: String,
    total_size: u64,
    uploaded_bytes: u64,
    cancellation_token: CancellationToken,
    stop_token: CancellationToken,
    progress_sender: Sender<ProgressTrack>,
    progress: ProgressTrack,
    last_measured_rate: Instant,
    reader_stream: ReaderStream<File>
}

impl UploadStream {
    pub async fn new (
        path: String,
        upload_id: Uuid,
        cancellation_token: CancellationToken,
        stop_token: CancellationToken,
        total_size: u64,
        progress: ProgressTrack,
        progress_sender: Sender<ProgressTrack>,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            upload_id: upload_id.to_string(),
            total_size,
            uploaded_bytes: 0,
            cancellation_token,
            progress_sender,
            progress,
            last_measured_rate: Instant::now(),
            stop_token,
            reader_stream: ReaderStream::new(File::open(path).await?)
        })
    }

    fn emit_progress(&mut self, chunk: &IoResult<Bytes>) {
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
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

impl Stream for UploadStream {
    type Item = IoResult<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.stop_token.is_cancelled() {
            debug!("upload stopped because maybe server returned an error for id: {}", self.upload_id);
            return Poll::Ready(None);
        }

        if self.cancellation_token.is_cancelled() {
            debug!("upload canceled for: {}", self.upload_id);
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.reader_stream).poll_next(cx) {
            Poll::Ready(Some(chunk)) => {
                self.emit_progress(&chunk);
                Poll::Ready(Some(chunk))
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending
        }
    }
}

pub trait UploadTask {
    fn get_progress_receiver(&self) -> Receiver<ProgressTrack>;
    fn get_cancellation_token(&self) -> CancellationToken;
    fn start(&self) -> JoinHandle<Result<(), FileUploadError>>;
}

#[derive(Debug, Clone)]
pub struct SingleUploadTask {
    base_url: String,
    progress_sender: Sender<ProgressTrack>,
    progress_receiver: Receiver<ProgressTrack>,
    cancellation_token: CancellationToken,
    stop_token: CancellationToken,
    client: reqwest::Client,
    upload: Upload,
    progress: ProgressTrack
}

impl SingleUploadTask {
    pub async fn new (
        base_url: String,
        upload: Upload,
        client: reqwest::Client,
    ) -> Result<Self, std::io::Error> {
        let progress = ProgressTrack::new(upload.id.clone(), upload.size);
        let cancellation_token = CancellationToken::new();
        let stop_token = CancellationToken::new();

        let (
            progress_sender,
            progress_receiver
        ) = watch::channel(progress.clone());

        Ok(Self {
            base_url,
            progress_sender,
            progress_receiver,
            cancellation_token,
            client,
            upload,
            stop_token,
            progress
        })
    }

    async fn create_stream(&self) -> Result<UploadStream, std::io::Error>  {
         UploadStream::new(
             self.upload.path.clone(),
             self.upload.id.clone(),
             self.cancellation_token.clone(),
             self.stop_token.clone(),
             self.upload.size,
             self.progress.clone(),
             self.progress_sender.clone(),
        ).await
    }

    async fn perform_upload(&self) -> Result<(), FileUploadError> {
        let stream = self.create_stream().await
            .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;
        let body = reqwest::Body::wrap_stream(stream);
        let url = format!("{}/api/file/{}/upload", self.base_url, self.upload.id);
        let send_future = self.client
            .post(&url)
            .header("Connection", "keep-alive")
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
        let status = response.status().clone();
        debug!("POST {}: {}", url, status);

        if status != 200 {
            self.stop_token.cancel();
            let response_body = response.text().await
                .map_err(|err| FileUploadError::ClientError { message: err.to_string()})?;
            debug!("POST {}: {} - {} (after stop file content stream)",  url, status, response_body);
            let error_response = ErrorResponse::from_string(response_body)
                .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;
            return Err(error_response.into());
        }

        Ok(())
    }
}

impl UploadTask for SingleUploadTask {
    fn get_progress_receiver(&self) -> Receiver<ProgressTrack> {
        self.progress_receiver.clone()
    }

    fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    fn start(&self) -> JoinHandle<Result<(), FileUploadError>> {
        let task = self.clone();

        tokio::spawn(async move {
            task.perform_upload().await
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChunkedUploadTask {
    base_url: String,
    progress_sender: Sender<ProgressTrack>,
    progress_receiver: Receiver<ProgressTrack>,
    cancellation_token: CancellationToken,
    progress: Arc<Mutex<ProgressTrack>>,
    client: reqwest::Client,
    upload: Upload,
    uploaded_bytes: Arc<AtomicU64>,
    last_measured_rate: Arc<Mutex<Instant>>,
    chunk_size: u64
}

impl ChunkedUploadTask {
    pub async fn new (
        base_url: String,
        upload: Upload,
        client: reqwest::Client,
        chunk_size: u64
    ) -> Result<Self, std::io::Error> {
        let progress = ProgressTrack::new(upload.id.clone(), upload.size);
        let cancellation_token = CancellationToken::new();

        let (
            progress_sender,
            progress_receiver
        ) = watch::channel(progress.clone());

        Ok(Self {
            base_url,
            progress_sender,
            progress_receiver,
            cancellation_token,
            client,
            upload,
            progress: Arc::new(Mutex::new(progress)),
            uploaded_bytes: Arc::new(AtomicU64::new(0)),
            last_measured_rate: Arc::new(Mutex::new(Instant::now())),
            chunk_size
        })
    }

    fn emit_progress(&self, bytes_sent: &std::io::Result<Bytes>) {
        let Ok(bytes_sent) = bytes_sent else {
            return
        };

        let bytes_sent = bytes_sent.len() as u64;
        let mut last_measured_rate = self.last_measured_rate.lock().unwrap();
        let uploaded_bytes = self.uploaded_bytes.load(std::sync::atomic::Ordering::Acquire);
        let uploaded_bytes_now = min(uploaded_bytes + bytes_sent, self.upload.size);
        self.uploaded_bytes.store(uploaded_bytes_now, std::sync::atomic::Ordering::Release);
        let elapsed = last_measured_rate.elapsed();

        if elapsed >= Duration::from_secs(1) {
            let mut progress = self.progress.lock().unwrap();
            let updated_progress = progress.update(uploaded_bytes_now);
            let _ = self.progress_sender.send(updated_progress);
            *last_measured_rate = Instant::now();
        }

        #[cfg(test)]
        {
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    async fn perform_chunk_upload(&self, data: Vec<u8>, index: u64) -> Result<(), FileUploadError> {
        let content_size = data.len();
        debug!("uploading chunk {} for file {} ({} bytes)", index, self.upload.id, content_size);
        let now = Instant::now();
        let url = format!("{}/api/file/{}/upload/part/{}", self.base_url, self.upload.id, index);
        let this = self.clone();
        let stream = ReaderStream::new(std::io::Cursor::new(data)).map(move |chunk| {
            this.emit_progress(&chunk);
            chunk
        });
        let body = reqwest::Body::wrap_stream(stream);
        let send_future = self.client
            .post(&url)
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", content_size)
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
        let status = response.status().clone();
        debug!("POST {}: {}", url, status);

        if status != 200 {
            let response_body = response.text().await
                .map_err(|err| FileUploadError::ClientError { message: err.to_string()})?;
            debug!("POST {}: {} - {} (after stop file content stream)", url, status, response_body);
            let error_response = ErrorResponse::from_string(response_body)
                .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;
            return Err(error_response.into());
        }

        debug!("uploaded chunk {} for file {} took {} ms", index, self.upload.id, now.elapsed().as_millis());
        Ok(())
    }

    async fn perform_upload(&mut self) -> Result<(), FileUploadError> {
        let mut file = File::open(self.upload.path.clone()).await
            .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;
        let chunk_size = self.chunk_size as usize;
        let mut buffer = vec![0u8; chunk_size];
        let mut index: u64 = 0;

        file.set_max_buf_size(chunk_size);

        loop {
            if self.cancellation_token.is_cancelled() {
                debug!("upload canceled for: {}", self.upload.id);
                return Ok(());
            }

            let bytes_read = file.read(&mut buffer).await
                .map_err(|err| FileUploadError::ClientError { message: err.to_string() })?;

            if bytes_read == 0 {
                debug!("finish uploading file chunks");
                return Ok(())
            }

            let chunk = &buffer[..bytes_read];
            debug!("attempting to upload chunk of {} bytes ({} bytes read by the reader)", chunk.len(), bytes_read);
            self.perform_chunk_upload(Vec::from(chunk), index).await?;
            index += 1;
        }
    }
}

impl UploadTask for ChunkedUploadTask {
    fn get_progress_receiver(&self) -> Receiver<ProgressTrack> {
        self.progress_receiver.clone()
    }

    fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    fn start(&self) -> JoinHandle<Result<(), FileUploadError>> {
        let mut task = self.clone();

        tokio::spawn(async move {
            task.perform_upload().await
        })
    }
}
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

use super::error::FileUploadError;
use super::response::ErrorResponse;
use crate::cipher::{Cipher, ENCRYPTION_BLOCK_SIZE};
use crate::upload::{Progress, Upload};
use bytes::{Bytes, BytesMut};
use log::debug;
use std::cmp::min;
use std::io::Result as IoResult;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::watch::Sender;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;

const READ_BUFFER_SIZE: usize = 64 * 1024;

struct FixedBufferFileReader {
    file: File,
    buffer: [u8; READ_BUFFER_SIZE],
}

impl FixedBufferFileReader {
    async fn open(path: &str) -> IoResult<Self> {
        Ok(Self {
            file: File::open(path).await?,
            buffer: [0u8; READ_BUFFER_SIZE],
        })
    }

    async fn read_block(&mut self, block_size: usize) -> IoResult<Option<Vec<u8>>> {
        let mut block = Vec::with_capacity(block_size);

        while block.len() < block_size {
            let remaining = block_size - block.len();
            let read_size = min(READ_BUFFER_SIZE, remaining);
            let bytes_read = self.file.read(&mut self.buffer[..read_size]).await?;

            if bytes_read == 0 {
                break;
            }

            block.extend_from_slice(&self.buffer[..bytes_read]);
        }

        if block.is_empty() {
            Ok(None)
        } else {
            Ok(Some(block))
        }
    }
}

async fn file_stream(
    upload: &Upload,
    cancellation: CancellationToken,
) -> IoResult<ReceiverStream<IoResult<Bytes>>> {
    let mut reader = FixedBufferFileReader::open(&upload.path).await?;

    // When encryption is enabled always will be an encryption key
    let cipher = upload
        .encryption_key
        .as_deref()
        .map(Cipher::new)
        .transpose()?;
    let (sender, receiver) = mpsc::channel(2);

    tokio::spawn(async move {
        loop {
            let block = select! {
                result = reader.read_block(ENCRYPTION_BLOCK_SIZE) => result,
                _ = cancellation.cancelled() => break,
                _ = sender.closed() => break,
            };

            let block = match block {
                Ok(Some(block)) => block,
                Ok(None) => break,
                Err(error) => {
                    let _ = sender.send(Err(error)).await;
                    break;
                }
            };

            let output = match &cipher {
                Some(cipher) => cipher.encrypt_block(&block),
                None => Ok(block),
            };

            let output = match output {
                Ok(output) => output,
                Err(error) => {
                    let _ = sender.send(Err(error)).await;
                    break;
                }
            };

            if sender.send(Ok(Bytes::from(output))).await.is_err() {
                break;
            }
        }
    });

    Ok(ReceiverStream::new(receiver))
}

#[derive(Debug, Clone)]
pub struct UploadContext {
    pub cancellation_token: CancellationToken,
    progress_sender: Sender<Progress>,
    uploaded_bytes: Arc<AtomicU64>,
    last_measured_rate: Arc<Mutex<Instant>>,
    progress: Arc<Mutex<Progress>>,
    total_size: u64,
}

impl UploadContext {
    pub fn new(
        upload: &Upload,
        cancellation_token: CancellationToken,
        progress_sender: Sender<Progress>,
    ) -> Self {
        Self {
            cancellation_token,
            progress_sender,
            progress: Arc::new(Mutex::new(Progress::from_upload(&upload))),
            uploaded_bytes: Arc::new(AtomicU64::new(0)),
            last_measured_rate: Arc::new(Mutex::new(Instant::now())),
            total_size: upload.size,
        }
    }

    pub fn emit_progress(&self, bytes_sent: &std::io::Result<Bytes>) {
        let Ok(bytes_sent) = bytes_sent else { return };

        let bytes_sent = bytes_sent.len() as u64;
        let mut last_measured_rate = self.last_measured_rate.lock().unwrap();
        let uploaded_bytes = self
            .uploaded_bytes
            .load(std::sync::atomic::Ordering::Acquire);
        let uploaded_bytes_now = min(uploaded_bytes + bytes_sent, self.total_size);
        self.uploaded_bytes
            .store(uploaded_bytes_now, std::sync::atomic::Ordering::Release);
        let measured_at = Instant::now();
        let elapsed = measured_at.duration_since(*last_measured_rate);

        if elapsed >= Duration::from_secs(1) {
            let mut progress = self.progress.lock().unwrap();
            let updated_progress = progress.update(uploaded_bytes_now, elapsed);
            let _ = self.progress_sender.send(updated_progress);
            *last_measured_rate = measured_at;
        }

        #[cfg(test)]
        {
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

pub trait UploadTask {
    async fn execute(&self) -> Result<(), FileUploadError>;
}

#[derive(Debug, Clone)]
pub struct SingleUploadTask {
    base_url: String,
    client: reqwest::Client,
    upload: Upload,
    context: UploadContext,
}

impl SingleUploadTask {
    pub fn new(
        base_url: String,
        upload: Upload,
        client: reqwest::Client,
        context: UploadContext,
    ) -> Self {
        Self {
            base_url,
            client,
            upload,
            context,
        }
    }

    async fn create_stream(
        &self,
    ) -> IoResult<impl tokio_stream::Stream<Item = IoResult<Bytes>> + use<>> {
        let context = self.context.clone();
        let stream = file_stream(&self.upload, self.context.cancellation_token.clone()).await?;

        Ok(stream.map(move |chunk| {
            context.emit_progress(&chunk);
            chunk
        }))
    }

    async fn perform_upload(&self) -> Result<(), FileUploadError> {
        let stream = self
            .create_stream()
            .await
            .map_err(|err| FileUploadError::ClientError {
                message: err.to_string(),
            })?;

        let body = reqwest::Body::wrap_stream(stream);
        let url = format!("{}/api/file/{}/upload", self.base_url, self.upload.id);

        let send_future = self
            .client
            .post(&url)
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", self.upload.encrypted_size())
            .body(body)
            .send();

        let response = select! {
            res = send_future => res.map_err(|err| {
                FileUploadError::ClientError { message: err.to_string() }
            })?,
            _ = self.context.cancellation_token.cancelled() => {
                return Err(FileUploadError::Canceled);
            }
        };

        let status = response.status().clone();
        debug!("POST {}: {}", url, status);

        if status != 200 {
            let response_body =
                response
                    .text()
                    .await
                    .map_err(|err| FileUploadError::ClientError {
                        message: err.to_string(),
                    })?;
            debug!(
                "POST {}: {} - {} (after stop file content stream)",
                url, status, response_body
            );
            let error_response = ErrorResponse::from_string(response_body).map_err(|err| {
                FileUploadError::ClientError {
                    message: err.to_string(),
                }
            })?;

            return Err(error_response.into());
        }

        Ok(())
    }
}

impl UploadTask for SingleUploadTask {
    async fn execute(&self) -> Result<(), FileUploadError> {
        self.perform_upload().await
    }
}

#[derive(Debug, Clone)]
pub struct ChunkedUploadTask {
    base_url: String,
    client: reqwest::Client,
    chunk_size: u64,
    context: UploadContext,
    upload: Upload,
}

impl ChunkedUploadTask {
    pub fn new(
        base_url: String,
        client: reqwest::Client,
        chunk_size: u64,
        context: UploadContext,
        upload: Upload,
    ) -> Self {
        Self {
            base_url,
            client,
            chunk_size,
            context,
            upload,
        }
    }

    async fn perform_chunk_upload(&self, data: Vec<u8>, index: u64) -> Result<(), FileUploadError> {
        let content_size = data.len();
        debug!(
            "uploading chunk {} for file {} ({} bytes)",
            index, self.upload.id, content_size
        );
        let now = Instant::now();
        let url = format!(
            "{}/api/file/{}/upload/part/{}",
            self.base_url, self.upload.id, index
        );

        let context = self.context.clone();
        let stream = ReaderStream::new(std::io::Cursor::new(data)).map(move |chunk| {
            context.emit_progress(&chunk);
            chunk
        });

        let body = reqwest::Body::wrap_stream(stream);

        let send_future = self
            .client
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
            _ = self.context.cancellation_token.cancelled() => {
                return Err(FileUploadError::Canceled);
            }
        };

        let status = response.status().clone();
        debug!("POST {}: {}", url, status);

        if status != 200 {
            let response_body =
                response
                    .text()
                    .await
                    .map_err(|err| FileUploadError::ClientError {
                        message: err.to_string(),
                    })?;
            debug!(
                "POST {}: {} - {} (after stop file content stream)",
                url, status, response_body
            );
            let error_response = ErrorResponse::from_string(response_body).map_err(|err| {
                FileUploadError::ClientError {
                    message: err.to_string(),
                }
            })?;

            return Err(error_response.into());
        }

        debug!(
            "uploaded chunk {} for file {} took {} ms",
            index,
            self.upload.id,
            now.elapsed().as_millis()
        );
        Ok(())
    }

    async fn perform_upload(&self) -> Result<(), FileUploadError> {
        if self.chunk_size == 0 || self.chunk_size > usize::MAX as u64 {
            return Err(FileUploadError::ClientError {
                message: "invalid upload part size".into(),
            });
        }

        let mut stream = file_stream(&self.upload, self.context.cancellation_token.clone())
            .await
            .map_err(|err| FileUploadError::ClientError {
                message: err.to_string(),
            })?;

        let part_size = self.chunk_size as usize;
        let mut pending = BytesMut::new();
        let mut index = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|err| FileUploadError::ClientError {
                message: err.to_string(),
            })?;

            pending.extend_from_slice(&chunk);

            while pending.len() >= part_size {
                let part = pending.split_to(part_size).to_vec();
                self.perform_chunk_upload(part, index).await?;

                index += 1;
            }
        }

        if !pending.is_empty() {
            self.perform_chunk_upload(pending.to_vec(), index).await?;
        }

        if self.context.cancellation_token.is_cancelled() {
            return Err(FileUploadError::Canceled);
        }

        self.acknowledge().await
    }

    async fn acknowledge(&self) -> Result<(), FileUploadError> {
        debug!("Acknowledging chunked upload: {}", self.upload.id);
        let url = format!("{}/api/file/{}/upload/ack", self.base_url, self.upload.id);

        let response = self
            .client
            .post(&url)
            .header("Connection", "keep-alive")
            .send()
            .await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(());
                }

                Err(FileUploadError::ClientError {
                    message: format!(
                        "upload ack responded with status {}",
                        response.status().as_u16()
                    ),
                })
            }
            Err(err) => Err(FileUploadError::ClientError {
                message: err.to_string(),
            }),
        }
    }
}

impl UploadTask for ChunkedUploadTask {
    async fn execute(&self) -> Result<(), FileUploadError> {
        self.perform_upload().await
    }
}

#[cfg(test)]
mod encryption_tests {
    use super::*;
    use crate::server::Server;

    #[tokio::test]
    async fn encrypted_stream_round_trips_across_transport_boundaries() {
        let path =
            std::env::temp_dir().join(format!("mikupush-encryption-{}", uuid::Uuid::new_v4()));
        let plaintext = vec![0x5a; ENCRYPTION_BLOCK_SIZE + 19];

        tokio::fs::write(&path, &plaintext).await.unwrap();

        let upload = Upload::new(
            uuid::Uuid::new_v4(),
            "test.bin".into(),
            plaintext.len() as u64,
            "application/octet-stream".into(),
            path.to_string_lossy().into_owned(),
            Server::test(),
            false,
            Some(crate::cipher::generate_key()),
        );

        assert_eq!(upload.encryption_key.as_deref().unwrap().len(), 64);
        assert!(upload.url.contains('#'));

        let mut stream = file_stream(&upload, CancellationToken::new())
            .await
            .unwrap();
        let mut wire = Vec::new();

        while let Some(chunk) = stream.next().await {
            wire.extend_from_slice(&chunk.unwrap());
        }

        assert_eq!(wire.len() as u64, upload.encrypted_size());

        let key = upload.encryption_key.as_deref().unwrap();
        let key = (0..32)
            .map(|index| u8::from_str_radix(&key[index * 2..index * 2 + 2], 16).unwrap())
            .collect::<Vec<_>>();
        use aes_gcm::aead::Aead;
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce};

        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let mut decrypted = Vec::new();

        for chunk in
            wire.chunks(ENCRYPTION_BLOCK_SIZE + crate::cipher::NONCE_SIZE + crate::cipher::TAG_SIZE)
        {
            decrypted.extend_from_slice(
                cipher
                    .decrypt(Nonce::from_slice(&chunk[..12]), &chunk[12..])
                    .unwrap()
                    .as_slice(),
            );
        }

        assert_eq!(decrypted, plaintext);

        tokio::fs::remove_file(path).await.unwrap();
    }
}

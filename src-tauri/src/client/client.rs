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

use super::error::{FileDeleteError, FileInfoError, FileUploadError, HealthCheckError};
use super::response::{ErrorResponse, FileInfo, HealthCheckStatus, ServerInfo};
use super::upload::SingleUploadTask;
use super::{ChunkedUploadTask, UploadContext, UploadTask};
use crate::server::Server;
use crate::upload::{Progress, Upload, UploadRequest};
use log::debug;
use serde_json::json;
use tokio::sync::watch::Sender;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const SINGLE_UPLOAD_MAX_SIZE_BYTES: u64 = 20971520; // 20MB
const SERVER_ICON_MAX_SIZE_BYTES: u64 = 2097152; // 2MB

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(server: Server) -> Self {
        debug!("using server client with base url: {}", server.url);
        Self {
            base_url: server.url,
            client: reqwest::Client::builder()
                .user_agent("MikuPush/1.0.0")
                .build()
                .unwrap(),
        }
    }

    pub async fn create(&self, upload: &Upload) -> Result<(), FileUploadError> {
        let data = json!({
            "id": upload.id,
            "name": upload.name,
            "mime_type": upload.mime_type,
            "size": upload.size
        });

        let url = format!("{}/api/file", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await
            .map_err(|err| FileUploadError::ClientError {
                message: err.to_string(),
            })?;
        let status = response.status().clone();
        let response_body = response
            .text()
            .await
            .map_err(|err| FileUploadError::ClientError {
                message: err.to_string(),
            })?;
        debug!("POST {}: {} - {}", url, status, response_body);

        if status != 200 {
            let error_response = ErrorResponse::from_string(response_body).map_err(|err| {
                FileUploadError::ClientError {
                    message: err.to_string(),
                }
            })?;
            return Err(error_response.into());
        }

        Ok(())
    }

    pub async fn info(&self, id: Uuid) -> Result<FileInfo, FileInfoError> {
        let url = format!("{}/api/file/{}", self.base_url, id);
        let response =
            self.client
                .delete(&url)
                .send()
                .await
                .map_err(|err| FileInfoError::ClientError {
                    message: err.to_string(),
                })?;
        let status = response.status().clone();
        let response_body = response
            .text()
            .await
            .map_err(|err| FileInfoError::ClientError {
                message: err.to_string(),
            })?;
        debug!("GET {}: {} - {}", url, status, response_body);

        if status != 200 {
            let error_response = ErrorResponse::from_string(response_body).map_err(|err| {
                FileInfoError::ClientError {
                    message: err.to_string(),
                }
            })?;
            return Err(error_response.into());
        }

        let info: FileInfo =
            serde_json::from_str(&response_body).map_err(|err| FileInfoError::ClientError {
                message: err.to_string(),
            })?;

        Ok(info)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), FileDeleteError> {
        let url = format!("{}/api/file/{}", self.base_url, id);
        let response =
            self.client
                .delete(&url)
                .send()
                .await
                .map_err(|err| FileDeleteError::ClientError {
                    message: err.to_string(),
                })?;
        let status = response.status().clone();
        let response_body = response
            .text()
            .await
            .map_err(|err| FileDeleteError::ClientError {
                message: err.to_string(),
            })?;
        debug!("DELETE {}: {} - {}", url, status, response_body);

        if status != 200 {
            let error_response = ErrorResponse::from_string(response_body).map_err(|err| {
                FileDeleteError::ClientError {
                    message: err.to_string(),
                }
            })?;
            return Err(error_response.into());
        }

        Ok(())
    }

    pub async fn upload(
        &self,
        request: &UploadRequest,
        cancellation_token: CancellationToken,
        sender: Sender<Progress>
    ) -> Result<(), FileUploadError> {
        if request.upload.mime_type.is_empty() {
            return Err(FileUploadError::UnknownMimeType);
        }

        let client = self.client.clone();
        let size = request.upload.size;
        let context = UploadContext::new(&request.upload, cancellation_token, sender);

        if request.chunked {
            debug!(
                "uploading file {} with size {} as chunked upload",
                request.upload.id, request.chunk_size
            );
            ChunkedUploadTask::new(
                self.base_url.clone(),
                client,
                request.chunk_size,
                context,
                request.upload.clone(),
            ).execute().await
        } else {
            debug!(
                "uploading file {} with size {} as single upload",
                request.upload.id, size
            );
            SingleUploadTask::new(
                self.base_url.clone(),
                request.upload.clone(),
                client,
                context
            ).execute().await
        }
    }

    pub async fn check_health(&self) -> Result<HealthCheckStatus, HealthCheckError> {
        let url = format!("{}/health?json", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|err| HealthCheckError {
                message: err.to_string(),
            })?;
        let status = response.status().clone();
        let response_body = response.text().await.map_err(|err| HealthCheckError {
            message: err.to_string(),
        })?;
        debug!("GET {}: {} - {}", url, status, response_body);

        HealthCheckStatus::from_string(response_body).map_err(|err| HealthCheckError {
            message: err.to_string(),
        })
    }

    pub async fn server_info(&self) -> Result<ServerInfo, String> {
        let url = format!("{}/api/info", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let status = response.status().clone();
        let response_body = response
            .text()
            .await
            .map_err(|err| format!("failed to retrieve response body: {}", err.to_string()))?;

        debug!("GET {}: {} - {}", url, status, response_body);

        if !status.is_success() {
            return Err(format!("Server responded with error: {}", response_body));
        }

        serde_json::from_str(&response_body)
            .map_err(|err| format!("failed to deserialize response: {}", err))
    }

    pub async fn server_icon(&self) -> Result<Option<(Vec<u8>, Option<String>)>, String> {
        let url = format!("{}/api/icon", self.base_url);
        let mut response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|err| format!("failed to retrieve server icon: {}", err.to_string()))?;

        let status = response.status().clone();
        debug!("GET {}: {}", url, status);

        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !status.is_success() {
            return Err(format!(
                "server responded with error status code: {}",
                status
            ));
        }

        let exceeds_max_size = response
            .content_length()
            .is_some_and(|size| size > SERVER_ICON_MAX_SIZE_BYTES);

        if exceeds_max_size {
            return Ok(None);
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);
        let mut bytes = Vec::new();

        let mut next_chunk = async || {
            response
                .chunk()
                .await
                .map_err(|err| format!("failed to retrieve server icon chunk: {}", err))
        };

        while let Some(chunk) = next_chunk().await? {
            if bytes.len() + chunk.len() > SERVER_ICON_MAX_SIZE_BYTES as usize {
                return Ok(None);
            }

            bytes.extend_from_slice(&chunk);
        }

        Ok(Some((bytes, content_type)))
    }
}

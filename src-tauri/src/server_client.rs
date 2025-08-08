use crate::models::{Upload, UploadRequest};
use crate::Result;
use log::debug;
use reqwest::Client;
use serde_json::json;
use std::cmp::min;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::fs::File;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

struct ProgressEvent {
    progress: f32,
    total_size: u64,
    uploaded_bytes: u64,
}

pub struct ServerClient {
    base_url: String,
    client: Client,
}

impl ServerClient {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        let base_url = "http://localhost:8080".to_string();

        #[cfg(not(debug_assertions))]
        let base_url = "https://mikupush.io".to_string();

        debug!("using server client with base url: {}", base_url);
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn create(&self, upload: &Upload) -> Result<()> {
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

    pub async fn delete(&self, id: &str) -> Result<()> {
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

    pub async fn upload<F>(&self, request: &UploadRequest, on_progress: F) -> Result<()>
    where
        F: Fn(ProgressEvent) + Send + Sync + 'static,
    {
        if request.upload.mime_type.is_empty() {
            return Err("unknown file type".into());
        }

        let file_path = Path::new(&request.upload.path);
        let file = File::open(file_path).await?;
        let total_size = file.metadata().await?.len();
        let mut reader_stream = ReaderStream::new(file);
        let uploaded_bytes = Arc::new(Mutex::new(0u64));

        let stream = async_stream::stream! {
            while let Some(chunk) = reader_stream.next().await {
                if let Ok(chunk) = &chunk {
                    let current_bytes: u64;
                    {
                        let mut bytes = uploaded_bytes.lock().unwrap();
                        *bytes = min(*bytes + (chunk.len() as u64),total_size);
                        current_bytes = *bytes;
                    }

                    on_progress(ProgressEvent{
                        progress: (current_bytes / total_size) as f32,
                        uploaded_bytes: current_bytes,
                        total_size: total_size
                    })
                }

                yield chunk;
            }
        };
        // TODO: separar a un struct y permitir cancelar
        let body = reqwest::Body::wrap_stream(stream);
        let url = format!("{}/api/file/{}/upload", self.base_url, request.upload.id);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", &request.upload.mime_type)
            .header("Content-Length", total_size.to_string())
            .body(body)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(format!(
                "error uploading file: {}",
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            )
            .into());
        }

        Ok(())
    }
}

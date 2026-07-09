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

use crate::UploadRequest;
use crate::server::Server;
use log::{debug, warn};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, LazyLock, Mutex, OnceLock};
use tauri::AppHandle;

static UPLOAD_QUEUE: LazyLock<UploadQueue> = LazyLock::new(|| UploadQueue::new());
static UPLOAD_WORKER_STARTED: OnceLock<()> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct UploadQueueJob {
    pub request: UploadRequest,
    pub server: Server,
    pub always_notify: bool,
    pub retry: bool,
}

#[derive(Debug, Clone)]
struct UploadQueue {
    queue: Arc<(Mutex<VecDeque<UploadQueueJob>>, Condvar)>,
}

impl UploadQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    pub fn get<'a>() -> &'a Self {
        &*UPLOAD_QUEUE
    }

    pub fn push(&self, item: UploadQueueJob) -> Result<(), String> {
        let (queue, queue_changed) = &*self.queue;
        let mut queue = match queue.lock() {
            Ok(queue) => queue,
            Err(err) => return Err(format!("error adding request to upload queue: {}", err)),
        };

        queue.push_back(item);
        queue_changed.notify_one();
        Ok(())
    }

    /// blocks until new [UploadRequest] is available
    pub fn pop_next_blocking(&self) -> UploadQueueJob {
        let (queue, queue_changed) = &*self.queue;
        let mut queue = queue.lock().unwrap_or_else(|err| {
            warn!("error locking queue for get last upload request: {}", err);
            err.into_inner()
        });

        loop {
            if let Some(item) = queue.pop_front() {
                return item;
            }

            queue = queue_changed.wait(queue).unwrap_or_else(|err| {
                warn!("error waiting for next upload request: {}", err);
                err.into_inner()
            });
        }
    }
}

pub fn enqueue_upload_job(item: UploadQueueJob) -> Result<(), String> {
    debug!("enqueue upload request {}", item.request.upload.id);
    UploadQueue::get().push(item)
}

pub fn start_upload_queue_worker(app_handle: AppHandle) -> Result<(), String> {
    if UPLOAD_WORKER_STARTED.set(()).is_err() {
        debug!("upload worker already started");
        return Ok(());
    }

    std::thread::Builder::new()
        .name("upload-worker".to_string())
        .spawn(move || {
            loop {
                let item = UploadQueue::get().pop_next_blocking();
                tauri::async_runtime::block_on(super::worker::process_queued_upload(
                    &app_handle,
                    item,
                ));
            }
        })
        .map_err(|err| err.to_string())?;

    Ok(())
}

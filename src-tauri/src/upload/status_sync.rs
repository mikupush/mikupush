use super::persistence::UploadRepository;
use super::status::Status;
use crate::AppContext;
use crate::client::{Client, FileRegistrationStatus};
use crate::database::{DbError, DbPool};
use crate::events::ARCHIVED_UPLOADS_CHANGED_EVENT;
use crate::server::ServerRepository;
use chrono::Utc;
use log::{debug, warn};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::{Mutex, watch};
use tokio::task::{JoinError, spawn_blocking};
use tokio::time::{Instant, sleep_until};
use uuid::Uuid;

const CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60);
const BATCH_SIZE: usize = 100;

pub struct ArchivedUploadStatusSync {
    server: watch::Sender<Option<Uuid>>,
    next_checks: Mutex<HashMap<Uuid, Instant>>,
}

impl ArchivedUploadStatusSync {
    pub fn new() -> Self {
        Self {
            server: watch::Sender::new(None),
            next_checks: Mutex::default(),
        }
    }

    pub fn set_server(&self, server_id: Option<Uuid>) {
        self.server.send_if_modified(|current| {
            if *current == server_id {
                return false;
            }
            *current = server_id;
            true
        });
    }
}

#[tauri::command]
pub fn set_archived_uploads_view(
    sync: State<'_, ArchivedUploadStatusSync>,
    server_id: Option<Uuid>,
) {
    sync.set_server(server_id);
}

#[tauri::command]
pub async fn sync_archived_upload_statuses(
    app: AppHandle,
    app_context: State<'_, AppContext>,
    sync: State<'_, ArchivedUploadStatusSync>,
    server_id: Uuid,
) -> Result<bool, String> {
    let Ok(mut next_checks) = sync.next_checks.try_lock() else {
        return Ok(false);
    };
    let pool = app_context
        .db_connection
        .get()
        .ok_or_else(|| "database connection is not initialized".to_string())?;

    sync_and_notify(&app, pool, server_id, true, &mut next_checks)
        .await
        .map_err(|error| error.to_string())?;
    Ok(true)
}

pub fn start_archived_upload_status_sync(app: AppHandle, pool: DbPool) {
    let mut view = app.state::<ArchivedUploadStatusSync>().server.subscribe();
    tauri::async_runtime::spawn(async move {
        let sync = app.state::<ArchivedUploadStatusSync>();
        loop {
            let server_id = *view.borrow_and_update();
            let Some(server_id) = server_id else {
                if view.changed().await.is_err() {
                    break;
                }
                continue;
            };

            let mut next_checks = sync.next_checks.lock().await;
            if *view.borrow() != Some(server_id) {
                continue;
            }

            let now = Instant::now();
            let next_check = next_checks.get(&server_id).copied().unwrap_or(now);
            if next_check <= now {
                let _ = sync_and_notify(&app, &pool, server_id, false, &mut next_checks).await;
                continue;
            }
            drop(next_checks);

            tokio::select! {
                changed = view.changed() => {
                    if changed.is_err() {
                        break;
                    }
                }
                _ = sleep_until(next_check) => {}
            }
        }
    });
}

async fn sync_and_notify(
    app: &AppHandle,
    pool: &DbPool,
    server_id: Uuid,
    force: bool,
    next_checks: &mut HashMap<Uuid, Instant>,
) -> Result<(), StatusSyncError> {
    let started_at = Instant::now();
    debug!("starting archived upload status sync for server {server_id} (manual: {force})");
    let result = sync_statuses(pool, server_id, force).await;
    match &result {
        Ok(()) => debug!(
            "finished archived upload status sync for server {server_id} in {} ms",
            started_at.elapsed().as_millis()
        ),
        Err(error) => warn!("could not refresh upload statuses for server {server_id}: {error}"),
    }
    next_checks.insert(server_id, Instant::now() + CHECK_INTERVAL);
    if let Err(error) = app.emit(ARCHIVED_UPLOADS_CHANGED_EVENT, server_id) {
        warn!("could not notify the upload history about the status check: {error}");
    }
    result
}

async fn sync_statuses(pool: &DbPool, server_id: Uuid, force: bool) -> Result<(), StatusSyncError> {
    let cutoff = (!force).then(|| (Utc::now() - CHECK_INTERVAL).naive_utc());
    let read_pool = pool.clone();
    let (server, ids) = spawn_blocking(move || {
        let server = ServerRepository::new(read_pool.clone()).find_by_id(server_id)?;
        let ids = UploadRepository::new(read_pool).ids_for_status_check(server_id, cutoff)?;
        Ok::<_, DbError>((server, ids))
    })
    .await??;
    let Some(server) = server else {
        return Ok(());
    };

    debug!(
        "found {} archived uploads due for a status check on server {server_id}",
        ids.len()
    );

    let client = Client::new(server);
    for (index, batch) in ids.chunks(BATCH_SIZE).enumerate() {
        let batch_started_at = Instant::now();
        let response = client.file_statuses(batch).await?;
        if !response
            .files
            .iter()
            .map(|file| file.id)
            .eq(batch.iter().copied())
        {
            return Err(StatusSyncError::UnexpectedIds);
        }

        let statuses: Vec<_> = response
            .files
            .into_iter()
            .map(|file| {
                let status = match file.status {
                    FileRegistrationStatus::Active => Status::Completed,
                    FileRegistrationStatus::Expired => Status::Expired,
                    FileRegistrationStatus::NotFound => Status::Deleted,
                };
                (file.id, status)
            })
            .collect();
        let checked_at = Utc::now().naive_utc();
        let write_pool = pool.clone();

        spawn_blocking(move || {
            UploadRepository::new(write_pool)
                .update_checked_statuses(server_id, &statuses, checked_at)
        })
        .await??;

        debug!(
            "synced archived upload status batch {} ({} uploads) for server {server_id} in {} ms",
            index + 1,
            batch.len(),
            batch_started_at.elapsed().as_millis()
        );
    }

    Ok(())
}

#[derive(Debug)]
enum StatusSyncError {
    Database(DbError),
    Task(JoinError),
    Request(reqwest::Error),
    UnexpectedIds,
}

impl From<DbError> for StatusSyncError {
    fn from(error: DbError) -> Self {
        Self::Database(error)
    }
}

impl From<JoinError> for StatusSyncError {
    fn from(error: JoinError) -> Self {
        Self::Task(error)
    }
}

impl From<reqwest::Error> for StatusSyncError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl fmt::Display for StatusSyncError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "{error}"),
            Self::Task(error) => write!(formatter, "{error}"),
            Self::Request(error) => write!(formatter, "{error}"),
            Self::UnexpectedIds => formatter.write_str("server returned unexpected upload IDs"),
        }
    }
}

impl std::error::Error for StatusSyncError {}

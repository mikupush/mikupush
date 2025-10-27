// Copyright 2025 Miku Push! Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod upload;
mod events;
mod state;
mod config;
mod resources;
mod server;
mod window;
mod menu;

use crate::resources::unpack_resources;
use crate::server::initialize_current_server_state;
use crate::window::{initialize_main_window, restore_main_window, MAIN_WINDOW};
use log::{debug, warn};
use mikupush_database::{create_database_connection, DbPool};
use state::{SelectedServerState, UploadsState};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tauri::image::Image;
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Emitter, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder, Wry};
use tauri_plugin_fs::FsExt;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use crate::menu::setup_app_menu;

pub struct AppContext {
    pub db_connection: OnceLock<DbPool>,
}

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

rust_i18n::i18n!("i18n", fallback = "en");

struct AppState {
    allow_quit: AtomicBool
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            allow_quit: AtomicBool::new(false)
        }
    }
}

// Initialize all plugins and set up the application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            debug!("single instance event");
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                debug!("restoring {} window from single instance event", MAIN_WINDOW);
                #[cfg(target_os = "windows")]
                sleep(Duration::from_millis(200)).await;
                restore_main_window(&app);
            });
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Error)
                .level_for("mikupush", log::LevelFilter::Debug)
                .level_for("mikupush_lib", log::LevelFilter::Debug)
                .level_for("mikupush_client", log::LevelFilter::Debug)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    })
                ])
                .build(),
        )
        .manage(AppContext {
            db_connection: OnceLock::new(),
        })
        .manage(UploadsState::new())
        .manage(SelectedServerState::new())
        .manage(AppState::default())
        .setup(|app| setup_app(app))
        // Register command handlers
        .invoke_handler(tauri::generate_handler![
            upload::select_files_to_upload,
            upload::enqueue_upload,
            upload::enqueue_many_uploads,
            upload::retry_upload,
            upload::delete_upload,
            upload::copy_upload_link,
            upload::cancel_upload,
            upload::get_all_in_progress_uploads,
            config::get_config_value,
            config::set_config_value,
            server::set_connected_server,
            server::get_connected_server,
            server::get_server_by_url,
            server::get_server_by_id,
            server::create_server,
            resources::server_icon_url,
            resources::resource_path,
            window::open_about_window
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                debug!("exit request");

                let state = app.state::<AppState>();
                if !state.allow_quit.load(Ordering::Relaxed) {
                    debug!("destroying window only and still run in background");
                    #[cfg(target_os = "macos")]
                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);

                    api.prevent_exit();
                }
            }
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => {
                debug!("reopen request");
                restore_main_window(app);
            }
            _ => {}
        });
}

fn setup_app(app: &mut App) -> GenericResult<()> {
    setup_app_menu(app.app_handle())?;
    unpack_resources(app.app_handle())?;
    let db = setup_app_database_connection(app);
    let app_context = app.state::<AppContext>();
    app_context.db_connection.set(db).unwrap();
    initialize_current_server_state(app.app_handle())?;
    initialize_main_window(app.app_handle());

    #[cfg(target_os = "macos")]
    let icon = Image::from(tauri::include_image!("icons/tray_icon.png"));
    #[cfg(target_os = "windows")]
    let icon = Image::from(tauri::include_image!("icons/tray_icon.ico"));
    #[cfg(target_os = "linux")]
    let icon = Image::from(tauri::include_image!("icons/tray_icon.png"));

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&setup_tray_menu(&app))
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| execute_tray_event(app, event))
        .build(app)?;

    Ok(())
}

fn setup_tray_menu(app: &App) -> Menu<Wry> {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    Menu::with_items(app, &[&show_item, &quit_item]).unwrap()
}

fn execute_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            let state = app.state::<AppState>();
            state.allow_quit.store(true, Ordering::Relaxed);
            app.exit(0);
        },
        "show" => {
            restore_main_window(app);
        }
        &_ => {}
    }
}

fn setup_app_database_connection(app: &App) -> DbPool {
    let app_dir = app.path().app_data_dir().unwrap();
    let database_file = app_dir.join("mikupush.db");
    debug!("sqlite database file: {:?}", database_file);

    if !database_file.exists() {
        debug!("sqlite database file does not exist, creating file...");
        std::fs::create_dir_all(app_dir).unwrap();
        std::fs::File::create(&database_file).unwrap();
        debug!("sqlite database file created on: {:?}", database_file);
    }

    let database_url = format!("sqlite://{}", database_file.to_str().unwrap());
    create_database_connection(&database_url)
}

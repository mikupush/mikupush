/// Copyright 2025 Miku Push! Team
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

mod commands;
mod events;
mod state;
mod settings;

use log::{debug, warn};
use state::{SelectedServerState, UploadsState};
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
use mikupush_database::{create_database_connection, DbPool};

pub struct AppContext {
    db_connection: OnceLock<DbPool>,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const MAIN_WINDOW_TITLE: &'static str = "MikuPush!";
const MAIN_WINDOW: &'static str = "main";

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
            commands::select_files_to_upload,
            commands::enqueue_upload,
            commands::enqueue_many_uploads,
            commands::retry_upload,
            commands::delete_upload,
            commands::copy_upload_link,
            commands::cancel_upload,
            commands::get_all_in_progress_uploads
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
    initialize_main_window(app.app_handle());

    {
        let db = setup_app_database_connection(app);
        let app_context = app.state::<AppContext>();
        app_context.db_connection.set(db).unwrap();
    }

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

fn initialize_main_window(app: &AppHandle) {
    let win_builder = WebviewWindowBuilder::new(app, MAIN_WINDOW, WebviewUrl::default())
        .title(MAIN_WINDOW_TITLE)
        .inner_size(800.0, 600.0);

    #[cfg(target_os = "windows")]
    {
        let window = win_builder.build().unwrap();
        window.set_decorations(false).unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        use objc2::rc::Retained;
        use objc2_app_kit::{NSWindow, NSWindowTitleVisibility, NSWindowToolbarStyle};
        use tauri::TitleBarStyle;

        let window = win_builder.build().unwrap();
        let ns_window_ptr = window.ns_window().unwrap();
        let obj_ptr = ns_window_ptr as *mut objc2::runtime::AnyObject;
        let ns_window: Retained<NSWindow> = unsafe { Retained::retain(obj_ptr.cast()) }.unwrap();

        window.set_title_bar_style(TitleBarStyle::Overlay).unwrap();

        unsafe {
            use objc2::{MainThreadMarker, MainThreadOnly};
            use objc2_app_kit::{NSToolbar, NSWindowCollectionBehavior};
            use objc2_foundation::NSString;

            let toolbar_id = NSString::from_str("MainToolbar");
            let mtm = MainThreadMarker::new().expect("must be on the main thread");
            let toolbar = NSToolbar::initWithIdentifier(NSToolbar::alloc(mtm), &toolbar_id);

            ns_window.setTitleVisibility(NSWindowTitleVisibility::Hidden);
            ns_window.setToolbar(Some(&toolbar));
            ns_window.setToolbarStyle(NSWindowToolbarStyle::Unified);
            ns_window.setCollectionBehavior(NSWindowCollectionBehavior::FullScreenNone);
        }
    }

    #[cfg(target_os = "linux")]
    {
        let _ = win_builder.build().unwrap();
    }
}

fn restore_main_window(app: &AppHandle) {
    debug!("attempting to restore {} window", MAIN_WINDOW);

    #[cfg(target_os = "macos")]
    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);

    let mut window = app.get_webview_window(MAIN_WINDOW);

    if window.is_none() {
        debug!("creating a new {} window instance because it was closed", MAIN_WINDOW);
        initialize_main_window(app);
        window = app.get_webview_window(MAIN_WINDOW);
    }

    if let Some(window) = window {
        debug!("restoring {} window instance", MAIN_WINDOW);
        let _ = window.show();
        let _ = window.set_focus();
    }
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
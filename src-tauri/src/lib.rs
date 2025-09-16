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
mod database;
mod events;
mod repository;
mod state;

use database::setup_app_database_connection;
use sea_orm::DatabaseConnection;
use state::{SelectedServerState, UploadsState};
use std::sync::Mutex;
use log::warn;
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry, RunEvent};
use tokio::runtime::Runtime;

pub struct AppContext {
    db_connection: Mutex<DatabaseConnection>,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const MAIN_WINDOW_TITLE: &'static str = "MikuPush!";
const MAIN_WINDOW: &'static str = "main";

rust_i18n::i18n!("i18n", fallback = "en");

// Initialize all plugins and set up the application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                #[cfg(not(target_os = "macos"))]
                {
                    if let Err(err) = window.hide() {
                        warn!("failed to hide window: {}", err)
                    }
                }

                #[cfg(target_os = "macos")]
                {
                    if let Err(err) = AppHandle::hide(&window.app_handle()) {
                        warn!("failed to hide window: {}", err)
                    }
                }

                api.prevent_close();
            }
            _ => {}
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let window = app.get_webview_window(MAIN_WINDOW).unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
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
            db_connection: Mutex::default(),
        })
        .manage(UploadsState::new())
        .manage(SelectedServerState::new())
        .setup(|app| setup_app(app))
        // Register command handlers
        .invoke_handler(tauri::generate_handler![
            commands::select_files_to_upload,
            commands::enqueue_upload,
            commands::enqueue_many_uploads,
            commands::retry_upload,
            commands::delete_upload,
            commands::copy_upload_link,
            commands::cancel_upload
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app(app: &mut App) -> GenericResult<()> {
    initialize_main_window(app);
    let tokio_runtime = Runtime::new().unwrap();
    let db = tokio_runtime.block_on(setup_app_database_connection(app));

    {
        let app_context = app.state::<AppContext>();
        let mut app_db_connection = app_context.db_connection.lock().unwrap();
        *app_db_connection = db;
    }

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&setup_tray_menu(&app))
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| execute_tray_event(app, event))
        .build(app)?;

    Ok(())
}

fn initialize_main_window(app: &App) {
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

fn setup_tray_menu(app: &App) -> Menu<Wry> {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    Menu::with_items(app, &[&show_item, &quit_item]).unwrap()
}

fn execute_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => app.exit(0),
        "show" => {
            let window = app.get_webview_window(MAIN_WINDOW).unwrap();
            window.show().unwrap();
            let _ = window.set_focus();
        }
        &_ => {}
    }
}

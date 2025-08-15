mod commands;
mod database;
mod events;
mod models;
mod repository;
mod server_client;
mod state;

use crate::models::UploadRequest;
use database::setup_app_database_connection;
use sea_orm::DatabaseConnection;
use state::UploadsState;
use std::sync::{Arc, Mutex};
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Manager, Wry};
use tokio::runtime::Runtime;

// TODO: crear un struct que se llame InProgressUploads que tenga un mapa key: uuid y valor UploadRequest
// y que este se vaya actualizando segun se aplica progreso en una subida o se termina
pub struct AppContext {
    db_connection: Mutex<DatabaseConnection>,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Initialize all plugins and set up the application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        // Set up the application state
        .manage(AppContext {
            db_connection: Mutex::default(),
        })
        .manage(UploadsState::new())
        .setup(|app| setup_app(app))
        // Register command handlers
        .invoke_handler(tauri::generate_handler![
            commands::select_files_to_upload,
            commands::enqueue_upload,
            commands::enqueue_many_uploads,
            commands::retry_upload,
            commands::abort_upload,
            commands::delete_upload,
            commands::find_all_uploads,
            commands::copy_upload_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app(app: &mut App) -> GenericResult<()> {
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

fn setup_tray_menu(app: &App) -> Menu<Wry> {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    Menu::with_items(app, &[&show_item, &quit_item]).unwrap()
}

fn execute_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => app.exit(0),
        "show" => {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap()
        }
        &_ => {}
    }
}

use rust_i18n::t;
use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use crate::GenericResult;
use crate::window::initialize_about_window;

const ABOUT_MENU_ITEM: &str = "about_app";

pub fn setup_app_menu(app_handle: &AppHandle) -> GenericResult<()> {
    let about_item = MenuItemBuilder::with_id(ABOUT_MENU_ITEM, t!("menu.about"))
        .build(app_handle)?;

    let app_submenu = SubmenuBuilder::new(app_handle, app_handle.package_info().clone().name)
        .items(&[
            &about_item,
        ])
        .build()?;

    let menu = MenuBuilder::new(app_handle)
        .items(&[&app_submenu])
        .build()?;

    app_handle.set_menu(menu)?;
    app_handle.on_menu_event(|app, event| {
        match event.id().0.as_str() {
            ABOUT_MENU_ITEM => {
                let _ = initialize_about_window(&app.app_handle());
            },
            _ => {}
        }
    });

    Ok(())
}

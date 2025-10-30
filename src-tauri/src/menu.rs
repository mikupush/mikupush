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
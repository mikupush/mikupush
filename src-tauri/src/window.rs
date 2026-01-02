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

use log::{debug, warn};
use rust_i18n::t;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const MAIN_WINDOW_TITLE: &'static str = "Miku Push!";
pub const ABOUT_WINDOW_TITLE: &'static str = "About Miku Push!";
pub const MAIN_WINDOW: &'static str = "main";
pub const ABOUT_WINDOW: &'static str = "about";

pub fn initialize_main_window(app: &AppHandle) {
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
        let window = win_builder.build().unwrap();
        let _ = window.remove_menu();
    }
}

pub fn restore_main_window(app: &AppHandle) {
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

pub fn initialize_about_window(app: &AppHandle) -> Result<(), String> {
    debug!("attempting to create {} window", ABOUT_WINDOW);
    let window = app.get_webview_window(ABOUT_WINDOW);
    if let Some(window) = window {
        let _ = window.show();
        let _ = window.set_focus();
        debug!("{} window is created and visible", ABOUT_WINDOW);
        return Ok(());
    }

    let win_builder = WebviewWindowBuilder::new(app, ABOUT_WINDOW, WebviewUrl::App("about.html".into()))
        .title(ABOUT_WINDOW_TITLE)
        .inner_size(800.0, 600.0)
        .min_inner_size(480.0, 600.0);

    let window = match win_builder.build() {
        Ok(window) => window,
        Err(err) => {
            warn!("error creating {} window: {}", ABOUT_WINDOW, err);
            return Err(t!("errors.window.open_about").to_string());
        }
    };

    let _ = window.remove_menu();

    debug!("{} window created", ABOUT_WINDOW);
    Ok(())
}

#[tauri::command]
pub fn open_about_window(app_handle: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        if let Err(err) = initialize_about_window(&app_handle) {
            warn!("error opening about window: {}", err);
        }
    });

    Ok(())
}
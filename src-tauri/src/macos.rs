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

#[cfg(target_os = "macos")]
pub fn get_group_container_path(group_id: &str) -> Option<String> {
    use objc2::rc::autoreleasepool;
    use objc2::runtime::NSObject;
    use objc2::{class, msg_send, sel};
    use objc2_foundation::{NSString, NSURL};

    autoreleasepool(|_| unsafe {
        // NSFileManager *fm = [NSFileManager defaultManager];
        let file_manager: *mut NSObject = msg_send![class!(NSFileManager), defaultManager];

        // NSString *group = @"group.com.tu.app";
        let group_nsstring = NSString::from_str(group_id);

        // NSURL *url = [fm containerURLForSecurityApplicationGroupIdentifier:group];
        let url: *mut NSURL = msg_send![
            file_manager,
            containerURLForSecurityApplicationGroupIdentifier: &*group_nsstring
        ];

        if url.is_null() {
            return None;
        }

        // NSString *path = [url path];
        let path: *mut NSString = msg_send![url, path];

        if path.is_null() {
            return None;
        }

        Some((*path).to_string())
    })
}
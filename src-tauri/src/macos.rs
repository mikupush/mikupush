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
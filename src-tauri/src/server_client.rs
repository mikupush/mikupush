use log::debug;

pub fn get_server_base_url() -> String {
    #[cfg(debug_assertions)]
    let base_url = "http://localhost:8080".to_string();

    #[cfg(not(debug_assertions))]
    let base_url = "https://mikupush.io".to_string();

    debug!("using server base url: {}", base_url);
    base_url
}

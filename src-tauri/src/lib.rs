mod requests;
mod stores;

use crate::requests::{send_get, send_post};
use std::sync::Arc;
use std::time::Duration;
use tauri::Manager;
use tauri_plugin_http::reqwest::cookie::Jar;
use tauri_plugin_http::reqwest::Client;

struct HttpClient {
    client: Client,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .cookie_provider(cookie_store)
        .build()
        .expect("Failed to build client");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // Secure Store
            let salt_path = app
                .app_handle()
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            Ok(())
        })
        .manage(HttpClient { client })
        .invoke_handler(tauri::generate_handler![send_post, send_get])
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

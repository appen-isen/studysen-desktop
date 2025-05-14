mod encryption;
mod requests;
mod stores;

use crate::requests::{send_get, send_post};
use crate::stores::{delete_secure_item, get_secure_item, set_secure_item};
use std::sync::Arc;
use std::time::Duration;
use tauri::Manager;
use tauri_plugin_http::reqwest::cookie::Jar;
use tauri_plugin_http::reqwest::Client;

struct AppState {
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
        .manage(AppState { client })
        .invoke_handler(tauri::generate_handler![
            send_post,
            send_get,
            get_secure_item,
            set_secure_item,
            delete_secure_item
        ])
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod aurion;

use std::sync::Arc;
use std::time::Duration;
use tauri_plugin_http::reqwest::cookie::Jar;
use tauri_plugin_http::reqwest::Client;

struct HttpClient {
    client: Client,
    base_url: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .cookie_provider(cookie_store)
        .build()
        .expect("Failed to build client");

    tauri::Builder::default()
        .manage(HttpClient {
            client,
            base_url: "https://web.isen-ouest.fr/webAurion".to_string(),
        })
        .invoke_handler(tauri::generate_handler![aurion::auth::login])
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

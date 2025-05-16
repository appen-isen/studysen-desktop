mod encryption;
mod requests;
mod stores;

use crate::requests::{send_get, send_post};
use crate::stores::{delete_item, get_item, get_secure_item, set_item, set_secure_item};
use std::sync::Arc;
use std::time::Duration;
use tauri_plugin_http::reqwest::cookie::Jar;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_updater::UpdaterExt;

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
        .setup(|app| {
            println!("Version: {}", app.package_info().version.to_string());
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            Ok(())
        })
        .manage(AppState { client })
        .invoke_handler(tauri::generate_handler![
            send_post,
            send_get,
            get_secure_item,
            set_secure_item,
            set_item,
            get_item,
            delete_item,
        ])
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//Mise à jour de l'application
async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        println!("Mise à jour disponible");
        let mut downloaded = 0;
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("Téléchargement {downloaded} sur {content_length:?}");
                },
                || {
                    println!("Téléchargement terminé");
                },
            )
            .await?;

        println!("Mise à jour terminée");
        app.restart();
    }

    Ok(())
}

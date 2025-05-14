use crate::encryption::{decrypt, encrypt};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn get_secure_item(app_handle: tauri::AppHandle, key: String) -> String {
    "".to_string()
}

#[tauri::command]
pub fn set_secure_item(app_handle: tauri::AppHandle, key: String, value: String) {}

#[tauri::command]
pub fn delete_secure_item(app_handle: tauri::AppHandle, key: String) {}

use crate::AppState;
use std::collections::HashMap;
use tauri::State;

// Requête POST
#[tauri::command]
pub async fn send_post(
    state: State<'_, AppState>,
    url: String,
    params: HashMap<String, String>,
) -> Result<String, String> {
    let response = state
        .client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}

// Requête GET
#[tauri::command]
pub async fn send_get(state: State<'_, AppState>, url: String) -> Result<String, String> {
    let response = state
        .client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}

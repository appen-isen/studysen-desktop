use crate::HttpClient;
use tauri::State;

// Requête de connexion
#[tauri::command]
pub async fn login(
    state: State<'_, HttpClient>,
    username: String,
    password: String,
) -> Result<String, String> {
    let params = [("username", username), ("password", password)];

    let url = format!("{}/login", state.base_url);
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

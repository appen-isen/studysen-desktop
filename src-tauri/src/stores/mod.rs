use crate::encryption::{decrypt, encrypt};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn get_secure_item(app_handle: tauri::AppHandle, key: String) -> Option<String> {
    if let Some(mut hex_string) = get_item(app_handle, key.clone()) {
        // Nettoyage : enlever les guillemets et les espaces
        hex_string = hex_string
            .trim_matches(|c| c == '"' || c == '\n' || c == '\r' || c == ' ')
            .to_string();
        // On transforme la chaîne hexadécimale en bytes
        let bytes: Result<Vec<u8>, _> = hex_string
            .trim_end_matches(',')
            .split(',')
            .map(|b| u8::from_str_radix(b.trim(), 16))
            .collect();

        if let Ok(bytes) = bytes {
            //On déchiffre les données
            match decrypt(
                &bytes,
                &[
                    66, 33, 32, 32, 66, 35, 65, 33, 32, 66, 37, 37, 64, 64, 39, 37, 64, 66, 61, 35,
                    64, 38, 36, 63, 62, 65, 61, 38, 34, 33, 65, 30,
                ],
            ) {
                Some(decrypted_bytes) => {
                    //On transforme le vecteur d'octets en chaîne de caractères
                    if let Ok(result_str) = String::from_utf8(decrypted_bytes) {
                        println!("Déchiffrement dans le store sécurisé: {}", key);
                        return Some(result_str);
                    }
                }
                None => {
                    println!("Erreur de déchiffrement: {}", key);
                }
            }
        }
    }
    None
}

#[tauri::command]
pub fn set_secure_item(app_handle: tauri::AppHandle, key: String, value: String) {
    //On chiffre les données
    let encrypted_data = encrypt(
        value.as_bytes(),
        &[
            66, 33, 32, 32, 66, 35, 65, 33, 32, 66, 37, 37, 64, 64, 39, 37, 64, 66, 61, 35, 64, 38,
            36, 63, 62, 65, 61, 38, 34, 33, 65, 30,
        ],
    );
    // Conversion de la chaîne de caractères en bytes
    let hex_string = encrypted_data
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(",");
    println!("Chiffrement dans le store sécurisé: {}", key);
    set_item(app_handle, key, hex_string);
}
#[tauri::command]
pub fn get_item(app_handle: tauri::AppHandle, key: String) -> Option<String> {
    if let Ok(store) = app_handle.store("store.json") {
        println!("Récupération dans le store: {}", key);
        if let Some(value) = store.get(key) {
            if let Some(s) = value.as_str() {
                return Some(s.to_string());
            }
        }
    }
    None
}

#[tauri::command]
pub fn set_item(app_handle: tauri::AppHandle, key: String, value: String) {
    if let Ok(store) = app_handle.store("store.json") {
        println!("Ajout dans le store: {}", key);
        store.set(key, value);
    }
}

#[tauri::command]
pub fn delete_item(app_handle: tauri::AppHandle, key: String) {
    if let Ok(store) = app_handle.store("store.json") {
        println!("Suppression dans le store: {}", key);
        store.delete(key);
    }
}

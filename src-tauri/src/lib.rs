#[tauri::command]
fn console_log(msg: String)-> String {
  println!("I was invoked from JavaScript, with this message: {}", msg);
  "Hello from Rust!".into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![console_log])
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

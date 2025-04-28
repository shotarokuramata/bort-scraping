mod fetch;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_body(date: &str) -> String {
    let date_str = date.replace("-", "");
    if let Err(e) = fetch::fetch_all_race_info(&date_str) {
        format!("an error occurred: {}", e)
    } else {
        format!("Success: {}", date)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_body])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod fetch;
mod parse;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_body(date: &str) -> String {
    let date_str = date.replace("-", "");

    let mut result = fetch::fetch_all_race_info(&date_str);
    if result.is_err() {
        return format!("an error occurred: {}", result.unwrap_err());
    } else {
    }

    let content = result.unwrap();

    let url_list = parse::get_url_list_from_race_index(&content);

    format!("Success: {}", "ALL")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_body])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

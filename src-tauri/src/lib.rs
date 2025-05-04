mod fetch;
mod headress;
mod parse {
    pub mod biyori;
    pub mod official;
}
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

    let url_suffix_list = parse::official::get_race_param_list_from_race_index(&content);

    format!("Success: {}", "ALL")
}

#[tauri::command]
fn get_shusso_info() -> String {
    let race_no = 1;
    let place_no = 2;
    let today = "20231001";
    let slider = 0;
    let result = headress::fetch_shusso_info_from_kyoteibiyori(race_no, place_no, today, slider);
    if result.is_err() {
        return format!("an error occurred: {}", result.unwrap_err());
    } else {
    }

    let mut win_rate_list = parse::biyori::get_win_rate_info(&result.unwrap());
    if win_rate_list.is_err() {
        return format!("an error occurred: {}", win_rate_list.unwrap_err());
    } else {
        return format!("Success: {:?}", win_rate_list.unwrap());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_body, get_shusso_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

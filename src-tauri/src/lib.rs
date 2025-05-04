mod fetch;
mod headress;
mod parse {
    pub mod biyori {
        pub mod flame;
    }
    pub mod official;
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_biyori_info(date: &str) -> String {
    let date_str = date.replace("-", "");

    let race_no = 1;
    let place_no = 2;
    let today = date_str;
    let slider = 1;
    let result = headress::fetch_shusso_info_from_kyoteibiyori(race_no, place_no, &today, slider);
    if result.is_err() {
        return format!("an error occurred: {}", result.unwrap_err());
    } else {
    }

    let race_data = parse::biyori::flame::get_escaped_flame_info(&result.unwrap());
    if race_data.is_err() {
        return format!("an error occurred: {}", race_data.unwrap_err());
    } else {
    }
    return format!("Success : {}", race_data.unwrap());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_biyori_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

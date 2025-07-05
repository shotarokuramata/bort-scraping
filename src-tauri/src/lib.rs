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
fn get_biyori_info(date: &str, race_number: &str, place_number: &str) -> String {
    let date_str = date.replace("-", "");

    let race_no = match race_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return format!("Invalid race number: {}", race_number),
    };
    let place_no = match place_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return format!("Invalid place number: {}", place_number),
    };
    let slider = 1; // 枠別情報
    let result =
        headress::fetch_shusso_info_from_kyoteibiyori(race_no, place_no, &date_str, slider);
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

mod fetch;
mod headress;
mod parse {
    pub mod biyori {
        pub mod flame;
        pub mod table_analyzer;
    }
    pub mod official;
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_biyori_info(date: &str, race_number: &str, place_number: &str) -> Result<parse::biyori::flame::RaceData, String> {
    let date_str = date.replace("-", "");

    let race_no = match race_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid race number: {}", race_number)),
    };
    let place_no = match place_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid place number: {}", place_number)),
    };
    let slider = 1; // 枠別情報
    let result =
        headress::fetch_shusso_info_from_kyoteibiyori(race_no, place_no, &date_str, slider);
    if result.is_err() {
        return Err(format!("an error occurred: {}", result.unwrap_err()));
    }

    let race_data = parse::biyori::flame::get_escaped_flame_info(&result.unwrap());
    match race_data {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("an error occurred: {}", err)),
    }
}

#[tauri::command]
fn get_odds_info(date: &str, race_number: &str, place_number: &str) -> Result<String, String> {
    let date_str = date.replace("-", "");

    let race_no = match race_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid race number: {}", race_number)),
    };
    let place_no = match place_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid place number: {}", place_number)),
    };
    
    let result = headress::fetch_odds_info_from_kyoteibiyori(race_no, place_no, &date_str);
    match result {
        Ok(html_content) => Ok(html_content),
        Err(err) => Err(format!("an error occurred: {}", err)),
    }
}

#[tauri::command]
fn get_parsed_odds_info(date: &str, race_number: &str, place_number: &str) -> Result<parse::biyori::flame::OddsData, String> {
    let date_str = date.replace("-", "");

    let race_no = match race_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid race number: {}", race_number)),
    };
    let place_no = match place_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid place number: {}", place_number)),
    };
    
    // HTMLを取得
    let html_result = headress::fetch_odds_info_from_kyoteibiyori(race_no, place_no, &date_str);
    let html_content = match html_result {
        Ok(content) => content,
        Err(err) => return Err(format!("HTML取得エラー: {}", err)),
    };
    
    // オッズデータを解析
    let odds_result = parse::biyori::flame::parse_odds_from_html(&html_content);
    match odds_result {
        Ok(odds_data) => Ok(odds_data),
        Err(err) => Err(format!("オッズ解析エラー: {}", err)),
    }
}

#[tauri::command]
fn get_win_place_odds_info(date: &str, race_number: &str, place_number: &str) -> Result<parse::biyori::flame::OddsData, String> {
    let date_str = date.replace("-", "");

    let race_no = match race_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid race number: {}", race_number)),
    };
    let place_no = match place_number.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid place number: {}", place_number)),
    };
    
    // 単勝・複勝HTMLを取得
    let html_result = headress::fetch_odds_info_from_kyoteibiyori(race_no, place_no, &date_str);
    let html_content = match html_result {
        Ok(content) => content,
        Err(err) => return Err(format!("単勝・複勝HTML取得エラー: {}", err)),
    };
    
    // 単勝・複勝オッズデータを解析
    let odds_result = parse::biyori::flame::parse_win_place_odds_from_html(&html_content);
    match odds_result {
        Ok(odds_data) => Ok(odds_data),
        Err(err) => Err(format!("単勝・複勝オッズ解析エラー: {}", err)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_biyori_info, get_odds_info, get_parsed_odds_info, get_win_place_odds_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

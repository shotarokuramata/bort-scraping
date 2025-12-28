use crate::services::scraping_service::ScrapingService;
use crate::models::race::{RaceData, OddsData, BulkRaceData};

#[tauri::command]
pub async fn get_biyori_info(
    date: &str,
    race_number: &str,
    place_number: &str,
) -> Result<RaceData, String> {
    let date = date.to_string();
    let race_number = race_number.to_string();
    let place_number = place_number.to_string();

    // 重い処理を別スレッドで実行
    tokio::task::spawn_blocking(move || {
        let race_no = race_number.parse::<u32>()
            .map_err(|_| format!("Invalid race number: {}", race_number))?;
        let place_no = place_number.parse::<u32>()
            .map_err(|_| format!("Invalid place number: {}", place_number))?;

        ScrapingService::get_race_info(&date, race_no, place_no)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

#[tauri::command]
pub fn get_odds_info(date: &str, race_number: &str, place_number: &str) -> Result<String, String> {
    let race_no = race_number.parse::<u32>()
        .map_err(|_| format!("Invalid race number: {}", race_number))?;
    let place_no = place_number.parse::<u32>()
        .map_err(|_| format!("Invalid place number: {}", place_number))?;

    ScrapingService::get_odds_html(date, race_no, place_no)
}

#[tauri::command]
pub async fn get_win_place_odds_info(
    date: &str,
    race_number: &str,
    place_number: &str,
) -> Result<OddsData, String> {
    let date = date.to_string();
    let race_number = race_number.to_string();
    let place_number = place_number.to_string();

    // 重い処理を別スレッドで実行
    tokio::task::spawn_blocking(move || {
        let race_no = race_number.parse::<u32>()
            .map_err(|_| format!("Invalid race number: {}", race_number))?;
        let place_no = place_number.parse::<u32>()
            .map_err(|_| format!("Invalid place number: {}", place_number))?;

        ScrapingService::get_win_place_odds(&date, race_no, place_no)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

#[tauri::command]
pub async fn get_bulk_race_data(
    window: tauri::Window,
    start_date: &str,
    end_date: &str,
    place_numbers: Vec<u32>,
    race_numbers: Vec<u32>,
) -> Result<Vec<BulkRaceData>, String> {
    ScrapingService::get_bulk_race_data(
        Some(window),
        start_date,
        end_date,
        place_numbers,
        race_numbers,
    ).await
}

#[tauri::command]
pub async fn scrape_html_from_url(url: String) -> Result<String, String> {
    // 重い処理を別スレッドで実行
    tokio::task::spawn_blocking(move || {
        ScrapingService::scrape_html(&url)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

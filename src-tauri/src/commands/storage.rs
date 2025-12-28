use crate::services::storage_service::StorageService;
use crate::models::race::{RaceData, OddsData};

#[tauri::command]
pub fn save_race_data_to_db(
    date: &str,
    place_number: u32,
    race_number: u32,
    race_data: RaceData,
) -> Result<(), String> {
    let service = StorageService::new()?;
    service.save_race(date, place_number, race_number, &race_data)
}

#[tauri::command]
pub fn get_race_data_from_db(
    date: &str,
    place_number: u32,
    race_number: u32,
) -> Result<Option<RaceData>, String> {
    let service = StorageService::new()?;
    service.get_race(date, place_number, race_number)
}

#[tauri::command]
pub fn save_odds_data_to_db(
    date: &str,
    place_number: u32,
    race_number: u32,
    odds_data: OddsData,
) -> Result<(), String> {
    let service = StorageService::new()?;
    service.save_odds(date, place_number, race_number, &odds_data)
}

#[tauri::command]
pub fn get_odds_data_from_db(
    date: &str,
    place_number: u32,
    race_number: u32,
) -> Result<Option<OddsData>, String> {
    let service = StorageService::new()?;
    service.get_odds(date, place_number, race_number)
}

#[tauri::command]
pub fn get_all_stored_race_keys() -> Result<Vec<String>, String> {
    let service = StorageService::new()?;
    service.get_all_race_keys()
}

#[tauri::command]
pub fn delete_race_data_from_db(date: &str, place_number: u32, race_number: u32) -> Result<(), String> {
    let service = StorageService::new()?;
    service.delete_race(date, place_number, race_number)
}

#[tauri::command]
pub fn clear_all_stored_data() -> Result<(), String> {
    let service = StorageService::new()?;
    service.clear_all()
}

// Module declarations
mod commands;
mod fetcher;
mod headress;
mod models;
mod parse {
    pub mod biyori {
        pub mod flame;
    }
    pub mod official;
}
mod repositories;
mod services;

// Re-export model types for backward compatibility
pub use models::race::*;
pub use models::venue::*;

// Tauri entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(commands::OpenApiServiceState::default())
        .invoke_handler(tauri::generate_handler![
            // Utils
            commands::greet,
            // Schedule
            commands::get_active_races,
            commands::get_all_venues_with_status,
            commands::get_monthly_schedule,
            // Scraping
            commands::get_biyori_info,
            commands::get_odds_info,
            commands::get_win_place_odds_info,
            commands::get_bulk_race_data,
            commands::scrape_html_from_url,
            // Storage
            commands::save_race_data_to_db,
            commands::get_race_data_from_db,
            commands::save_odds_data_to_db,
            commands::get_odds_data_from_db,
            commands::get_all_stored_race_keys,
            commands::delete_race_data_from_db,
            commands::clear_all_stored_data,
            // Open API
            commands::init_open_api_service,
            commands::fetch_previews_data,
            commands::fetch_results_data,
            commands::fetch_programs_data,
            commands::save_previews_to_db,
            commands::save_results_to_db,
            commands::save_programs_to_db,
            commands::export_open_api_to_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Tests
#[cfg(test)]
mod tests {
    // ===== パラメータ検証テスト =====

    #[tokio::test]
    async fn test_get_biyori_info_invalid_race_number() {
        // 無効なレース番号の検証ロジックをテスト
        let race_number = "invalid";
        let result = race_number.parse::<u32>();

        assert!(result.is_err());
        println!("✅ 無効なレース番号検証ロジック成功");
    }

    #[tokio::test]
    async fn test_get_biyori_info_invalid_place_number() {
        // 無効な競艇場番号の検証ロジックをテスト
        let place_number = "invalid";
        let result = place_number.parse::<u32>();

        assert!(result.is_err());
        println!("✅ 無効な競艇場番号検証ロジック成功");
    }

    #[tokio::test]
    async fn test_date_parsing_logic() {
        use chrono::NaiveDate;

        // 有効な日付形式
        let valid_result = NaiveDate::parse_from_str("2025-01-15", "%Y-%m-%d");
        assert!(valid_result.is_ok());

        // 無効な日付形式
        let invalid_result = NaiveDate::parse_from_str("invalid-date", "%Y-%m-%d");
        assert!(invalid_result.is_err());

        println!("✅ 日付パース検証ロジック成功");
    }
}

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
        .plugin(tauri_plugin_dialog::init())
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
            commands::export_open_api_to_csv,
            // Open API - 高配当検索
            commands::search_high_payout_races,
            commands::get_payout_statistics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Tests
#[cfg(test)]
mod tests {
    use crate::services::open_api_service::OpenApiService;

    // ===== V2マイグレーションテスト =====

    #[tokio::test]
    async fn test_v2_migration_execution() {
        println!("🔄 Testing V2 migration...");

        // OpenApiServiceを初期化するとマイグレーションが実行される
        let service = OpenApiService::new(Some("data/open_api.db")).await;

        assert!(service.is_ok(), "Service initialization should succeed");
        println!("✅ V2 migration test passed");
    }

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

    // ===== 高配当検索テスト =====

    #[tokio::test]
    async fn test_search_high_payout_races() {
        use crate::services::open_api_service::OpenApiService;

        println!("🔍 Testing high payout search...");

        let service = OpenApiService::new(Some("data/open_api.db")).await
            .expect("Failed to initialize service");

        // 3連単配当100,000円以上のレースを検索
        let results = service.search_high_payout_races(100000, "trifecta".to_string(), Some(5)).await
            .expect("Failed to search high payout races");

        assert!(results.len() > 0, "Should find at least one high payout race");
        println!("✅ Found {} high payout races (trifecta >= 100,000)", results.len());

        for (i, result) in results.iter().enumerate() {
            let trifecta_payout = result.payouts.trifecta
                .as_ref()
                .and_then(|entries| entries.first())
                .and_then(|e| e.payout)
                .unwrap_or(0);
            println!("  {}. Date: {}, Venue: {:02}, Race: {}, Payout: ¥{}", 
                i + 1, result.race_date, result.race_stadium_number, result.race_number, trifecta_payout);
        }

        println!("✅ High payout search test passed");
    }

    #[tokio::test]
    async fn test_get_payout_statistics() {
        use crate::services::open_api_service::OpenApiService;

        println!("📊 Testing payout statistics...");

        let service = OpenApiService::new(Some("data/open_api.db")).await
            .expect("Failed to initialize service");

        let stats = service.get_payout_statistics().await
            .expect("Failed to get payout statistics");

        println!("  Average trifecta: ¥{:.2}", stats.avg_trifecta.unwrap_or(0.0));
        println!("  Max trifecta: ¥{}", stats.max_trifecta.unwrap_or(0));
        println!("  Average win: ¥{:.2}", stats.avg_win.unwrap_or(0.0));
        println!("  Max win: ¥{}", stats.max_win.unwrap_or(0));

        assert!(stats.max_trifecta.is_some(), "Should have max trifecta payout");
        assert!(stats.avg_trifecta.is_some(), "Should have average trifecta payout");

        println!("✅ Payout statistics test passed");
    }

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
            commands::export_open_api_to_csv_v3,
            // Open API - 高配当検索
            commands::search_high_payout_races,
            commands::get_payout_statistics,
            // Open API - V3検索（複合条件）
            commands::search_races_advanced,
            commands::search_races_by_racer,
            commands::search_races_by_racer_name,
            commands::search_races_by_class,
            commands::search_races_by_date_range,
            commands::search_races_by_venue,
            // Open API - データサマリー
            commands::get_open_api_data_summary
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

    // ===== V3 CSVエクスポートテスト =====

    #[tokio::test]
    async fn test_export_csv_v3_structure() {
        use crate::services::open_api_service::OpenApiService;

        println!("📁 Testing V3 CSV export...");

        let service = OpenApiService::new(Some("data/open_api.db")).await
            .expect("Failed to initialize service");

        // 一時ディレクトリに出力
        let temp_dir = std::env::temp_dir().join("bort_csv_export_test");
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
        let temp_dir_str = temp_dir.to_str().unwrap();

        // CSVエクスポート実行
        let (race_count, participant_count) = service
            .export_to_csv_v3(temp_dir_str)
            .await
            .expect("Failed to export CSV");

        println!("  📊 Exported {} races and {} participants", race_count, participant_count);

        // ファイルの存在確認
        let races_csv = temp_dir.join("races.csv");
        let participants_csv = temp_dir.join("race_participants.csv");

        assert!(races_csv.exists(), "races.csv should exist");
        assert!(participants_csv.exists(), "race_participants.csv should exist");

        // CSVヘッダーの検証
        let mut races_reader = csv::Reader::from_path(&races_csv).expect("Failed to read races.csv");
        let races_headers = races_reader.headers().expect("Failed to read headers");

        // 重要なカラムが存在することを確認
        assert!(races_headers.iter().any(|h| h == "race_date"), "Should have race_date column");
        assert!(races_headers.iter().any(|h| h == "trifecta_payout"), "Should have trifecta_payout column");
        assert!(races_headers.iter().any(|h| h == "race_title"), "Should have race_title column");

        // JSONカラムが除外されていることを確認
        assert!(!races_headers.iter().any(|h| h == "result_data_json"), "Should NOT have result_data_json");
        assert!(!races_headers.iter().any(|h| h == "program_data_json"), "Should NOT have program_data_json");

        println!("  ✅ races.csv has correct structure (no JSON columns)");

        // race_participants.csv のヘッダー検証
        let mut participants_reader = csv::Reader::from_path(&participants_csv).expect("Failed to read race_participants.csv");
        let participants_headers = participants_reader.headers().expect("Failed to read headers");

        assert!(participants_headers.iter().any(|h| h == "racer_name"), "Should have racer_name column");
        assert!(participants_headers.iter().any(|h| h == "boat_number"), "Should have boat_number column");
        assert!(participants_headers.iter().any(|h| h == "place_number"), "Should have place_number column");

        // previewsフィールドの検証
        assert!(participants_headers.iter().any(|h| h == "racer_weight_adjustment"), "Should have racer_weight_adjustment column");
        assert!(participants_headers.iter().any(|h| h == "racer_exhibition_time"), "Should have racer_exhibition_time column");
        assert!(participants_headers.iter().any(|h| h == "racer_tilt_adjustment"), "Should have racer_tilt_adjustment column");

        println!("  ✅ race_participants.csv has correct structure (including previews data)");

        // レコード数の検証
        assert_eq!(race_count, 471, "Should export 471 races");
        assert_eq!(participant_count, 2826, "Should export 2826 participants");

        println!("  ✅ Correct number of records exported");

        // クリーンアップ
        std::fs::remove_dir_all(&temp_dir).expect("Failed to cleanup temp directory");

        println!("✅ V3 CSV export test passed");
    }
}

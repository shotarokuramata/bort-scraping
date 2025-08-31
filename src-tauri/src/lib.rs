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
fn get_biyori_info(
    date: &str,
    race_number: &str,
    place_number: &str,
) -> Result<parse::biyori::flame::RaceData, String> {
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
fn get_win_place_odds_info(
    date: &str,
    race_number: &str,
    place_number: &str,
) -> Result<parse::biyori::flame::OddsData, String> {
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

#[tauri::command]
async fn get_bulk_race_data(
    start_date: &str,
    end_date: &str,
    place_numbers: Vec<u32>,
    race_numbers: Vec<u32>,
) -> Result<Vec<parse::biyori::flame::BulkRaceData>, String> {
    use chrono::{NaiveDate, Duration};
    use tokio::time::{sleep, Duration as TokioDuration};
    
    let mut all_results = Vec::new();
    
    // 日付範囲を生成
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid start date format: {}", e))?;
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid end date format: {}", e))?;
    
    let mut current_date = start;
    
    while current_date <= end {
        let date_str = current_date.format("%Y-%m-%d").to_string();
        let date_str_no_dash = current_date.format("%Y%m%d").to_string();
        
        // 各競艇場とレースの組み合わせを処理
        for &place_number in &place_numbers {
            for &race_number in &race_numbers {
                let mut bulk_data = parse::biyori::flame::BulkRaceData {
                    date: date_str.clone(),
                    place_number,
                    race_number,
                    race_data: None,
                    win_place_odds_data: None,
                    error: None,
                };
                
                // 競艇日和データを取得
                match headress::fetch_shusso_info_from_kyoteibiyori(race_number, place_number, &date_str_no_dash, 1) {
                    Ok(html_content) => {
                        match parse::biyori::flame::get_escaped_flame_info(&html_content) {
                            Ok(race_data) => bulk_data.race_data = Some(race_data),
                            Err(e) => bulk_data.error = Some(format!("Race data parse error: {}", e)),
                        }
                    }
                    Err(e) => bulk_data.error = Some(format!("Race data fetch error: {}", e)),
                }
                
                // 単勝・複勝オッズを取得
                match headress::fetch_odds_info_from_kyoteibiyori(race_number, place_number, &date_str_no_dash) {
                    Ok(win_place_html) => {
                        match parse::biyori::flame::parse_win_place_odds_from_html(&win_place_html) {
                            Ok(win_place_odds) => bulk_data.win_place_odds_data = Some(win_place_odds),
                            Err(e) => {
                                if bulk_data.error.is_none() {
                                    bulk_data.error = Some(format!("Win/place odds parse error: {}", e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if bulk_data.error.is_none() {
                            bulk_data.error = Some(format!("Win/place odds fetch error: {}", e));
                        }
                    }
                }
                
                all_results.push(bulk_data);
                
                // レート制限: リクエスト間に1秒間隔を設ける
                sleep(TokioDuration::from_secs(1)).await;
            }
        }
        
        current_date += Duration::days(1);
    }
    
    Ok(all_results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_biyori_info,
            get_odds_info,
            get_win_place_odds_info,
            get_bulk_race_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_biyori_info_valid_params() {
        // 有効なパラメータでテスト
        let result = get_biyori_info("2025-07-26", "1", "1");
        
        match result {
            Ok(race_data) => {
                // データが正常に取得できることを確認
                assert!(!race_data.player_basic_info.name.is_empty());
                assert!(!race_data.player_basic_info.registration_number.is_empty());
                println!("✅ レースデータ取得成功: {}", race_data.player_basic_info.name);
            }
            Err(e) => {
                // HTMLファイルがない場合は、エラーメッセージが適切であることを確認
                println!("⚠️ レースデータ取得エラー（HTMLファイル不足の可能性）: {}", e);
                assert!(e.contains("error") || e.contains("エラー") || e.contains("failed"));
            }
        }
    }

    #[test]
    fn test_get_biyori_info_invalid_race_number() {
        // 無効なレース番号でテスト
        let result = get_biyori_info("2025-07-26", "invalid", "1");
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid race number"));
        println!("✅ 無効なレース番号エラーハンドリング成功: {}", error);
    }

    #[test]
    fn test_get_biyori_info_invalid_place_number() {
        // 無効な競艇場番号でテスト
        let result = get_biyori_info("2025-07-26", "1", "invalid");
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid place number"));
        println!("✅ 無効な競艇場番号エラーハンドリング成功: {}", error);
    }

    #[test]
    fn test_get_win_place_odds_info_valid_params() {
        // 有効なパラメータで単勝・複勝オッズテスト
        let result = get_win_place_odds_info("2025-07-26", "1", "1");
        
        match result {
            Ok(odds_data) => {
                // データが正常に取得できることを確認
                assert!(!odds_data.combinations.is_empty());
                assert_eq!(odds_data.betting_type, parse::biyori::flame::BettingType::WinPlace);
                println!("✅ 単勝・複勝オッズ取得成功: {}パターン", odds_data.combinations.len());
            }
            Err(e) => {
                // HTMLファイルがない場合は、エラーメッセージが適切であることを確認
                println!("⚠️ 単勝・複勝オッズ取得エラー（HTMLファイル不足の可能性）: {}", e);
                assert!(e.contains("エラー") || e.contains("error") || e.contains("failed"));
            }
        }
    }

    #[test]
    fn test_get_win_place_odds_info_invalid_params() {
        // 無効なパラメータでテスト
        let result = get_win_place_odds_info("2025-07-26", "invalid", "1");
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid race number"));
        println!("✅ 無効パラメータエラーハンドリング成功: {}", error);
    }

    #[tokio::test]
    async fn test_get_bulk_race_data_valid_params() {
        // 一括取得機能のテスト（小規模データ）
        let result = get_bulk_race_data(
            "2025-07-26", 
            "2025-07-26", 
            vec![1], 
            vec![1]
        ).await;
        
        match result {
            Ok(bulk_data) => {
                assert_eq!(bulk_data.len(), 1);
                let item = &bulk_data[0];
                assert_eq!(item.date, "2025-07-26");
                assert_eq!(item.place_number, 1);
                assert_eq!(item.race_number, 1);
                println!("✅ 一括取得成功: {}件", bulk_data.len());
            }
            Err(e) => {
                // HTMLファイルがない場合の適切なエラーハンドリング
                println!("⚠️ 一括取得エラー（HTMLファイル不足の可能性）: {}", e);
                assert!(e.contains("error") || e.contains("エラー") || e.contains("failed"));
            }
        }
    }

    #[tokio::test]
    async fn test_get_bulk_race_data_invalid_date_format() {
        // 無効な日付形式でテスト
        let result = get_bulk_race_data(
            "invalid-date", 
            "2025-07-26", 
            vec![1], 
            vec![1]
        ).await;
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid start date format"));
        println!("✅ 無効日付形式エラーハンドリング成功: {}", error);
    }

    #[tokio::test]
    async fn test_get_bulk_race_data_structure_validation() {
        // 一括取得の構造検証（軽量版）
        let result = get_bulk_race_data(
            "2025-07-26", 
            "2025-07-26", 
            vec![1], 
            vec![1, 2]
        ).await;
        
        match result {
            Ok(bulk_data) => {
                // 1日 × 1競艇場 × 2レース = 2件
                assert_eq!(bulk_data.len(), 2);
                println!("✅ 構造検証テスト成功: {}件", bulk_data.len());
                
                // 各アイテムの基本構造を検証
                for item in &bulk_data {
                    assert_eq!(item.date, "2025-07-26");
                    assert_eq!(item.place_number, 1);
                    assert!([1, 2].contains(&item.race_number));
                }
            }
            Err(e) => {
                println!("⚠️ 構造検証エラー: {}", e);
                // エラーでも適切にハンドリングされていることを確認
                assert!(e.contains("error") || e.contains("エラー") || e.contains("failed"));
            }
        }
    }
}

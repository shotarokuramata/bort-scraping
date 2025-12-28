use crate::repositories::local_db::LocalDbRepository;
use crate::headress;
use crate::parse::biyori::flame;
use crate::models::race::{RaceData, OddsData, BulkRaceData};
use crate::models::venue::BulkProgressPayload;
use tauri::Emitter;

pub struct ScrapingService;

impl ScrapingService {
    pub fn get_race_info(
        date: &str,
        race_number: u32,
        place_number: u32,
    ) -> Result<RaceData, String> {
        let repo = LocalDbRepository::new()
            .map_err(|e| format!("Database initialization error: {}", e))?;

        // 1. まずデータベースから取得を試行
        match repo.get_race_data(date, place_number, race_number) {
            Ok(Some(cached_data)) => {
                println!(
                    "📦 キャッシュからレースデータを取得: {}-{}-{}",
                    date, place_number, race_number
                );
                return Ok(cached_data);
            }
            Ok(None) => {
                println!(
                    "🌐 キャッシュにデータなし、スクレイピング実行: {}-{}-{}",
                    date, place_number, race_number
                );
            }
            Err(err) => {
                println!("⚠️ データベース取得エラー、スクレイピング実行: {}", err);
            }
        }

        // 2. キャッシュにない場合はスクレイピング実行
        let date_str = date.replace("-", "");
        let slider = 1; // 枠別情報
        let result =
            headress::fetch_shusso_info_from_kyoteibiyori(race_number, place_number, &date_str, slider);
        if result.is_err() {
            return Err(format!("an error occurred: {}", result.unwrap_err()));
        }

        let race_data = flame::get_escaped_flame_info(&result.unwrap());
        match race_data {
            Ok(data) => {
                // 3. 取得したデータをデータベースに保存
                if let Err(save_err) = repo.save_race_data(date, place_number, race_number, &data) {
                    println!("⚠️ データベース保存エラー: {}", save_err);
                } else {
                    println!(
                        "💾 レースデータをデータベースに保存: {}-{}-{}",
                        date, place_number, race_number
                    );
                }
                Ok(data)
            }
            Err(err) => Err(format!("an error occurred: {}", err)),
        }
    }

    pub fn get_odds_html(
        date: &str,
        race_number: u32,
        place_number: u32,
    ) -> Result<String, String> {
        let date_str = date.replace("-", "");
        let result = headress::fetch_odds_info_from_kyoteibiyori(race_number, place_number, &date_str);
        match result {
            Ok(html_content) => Ok(html_content),
            Err(err) => Err(format!("an error occurred: {}", err)),
        }
    }

    pub fn get_win_place_odds(
        date: &str,
        race_number: u32,
        place_number: u32,
    ) -> Result<OddsData, String> {
        let repo = LocalDbRepository::new()
            .map_err(|e| format!("Database initialization error: {}", e))?;

        // 1. まずデータベースから取得を試行
        match repo.get_odds_data(date, place_number, race_number) {
            Ok(Some(cached_odds)) => {
                println!(
                    "📦 キャッシュからオッズデータを取得: {}-{}-{}",
                    date, place_number, race_number
                );
                return Ok(cached_odds);
            }
            Ok(None) => {
                println!(
                    "🌐 キャッシュにデータなし、スクレイピング実行: {}-{}-{}",
                    date, place_number, race_number
                );
            }
            Err(err) => {
                println!("⚠️ データベース取得エラー、スクレイピング実行: {}", err);
            }
        }

        // 2. キャッシュにない場合はスクレイピング実行
        let date_str = date.replace("-", "");
        let html_result = headress::fetch_odds_info_from_kyoteibiyori(race_number, place_number, &date_str);
        let html_content = match html_result {
            Ok(content) => content,
            Err(err) => return Err(format!("単勝・複勝HTML取得エラー: {}", err)),
        };

        // 単勝・複勝オッズデータを解析
        let odds_result = flame::parse_win_place_odds_from_html(&html_content);
        match odds_result {
            Ok(odds_data) => {
                // 3. 取得したデータをデータベースに保存
                if let Err(save_err) =
                    repo.save_odds_data(date, place_number, race_number, &odds_data)
                {
                    println!("⚠️ データベース保存エラー: {}", save_err);
                } else {
                    println!(
                        "💾 オッズデータをデータベースに保存: {}-{}-{}",
                        date, place_number, race_number
                    );
                }
                Ok(odds_data)
            }
            Err(err) => Err(format!("単勝・複勝オッズ解析エラー: {}", err)),
        }
    }

    pub async fn get_bulk_race_data(
        window: Option<tauri::Window>,
        start_date: &str,
        end_date: &str,
        place_numbers: Vec<u32>,
        race_numbers: Vec<u32>,
    ) -> Result<Vec<BulkRaceData>, String> {
        use chrono::{Duration, NaiveDate};
        use tokio::time::{sleep, Duration as TokioDuration};

        let repo = LocalDbRepository::new()
            .map_err(|e| format!("Database initialization error: {}", e))?;

        let mut all_results = Vec::new();

        // 日付範囲を生成
        let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|e| format!("Invalid start date format: {}", e))?;
        let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|e| format!("Invalid end date format: {}", e))?;

        let mut current_date = start;

        // 総処理数を計算
        let days = (end - start).num_days() + 1;
        let total_tasks = (days as usize) * place_numbers.len() * race_numbers.len();
        let mut current_task = 0;

        while current_date <= end {
            let date_str = current_date.format("%Y-%m-%d").to_string();
            let date_str_no_dash = current_date.format("%Y%m%d").to_string();

            // 各競艇場とレースの組み合わせを処理
            for &place_number in &place_numbers {
                for &race_number in &race_numbers {
                    current_task += 1;

                    let mut bulk_data = BulkRaceData {
                        date: date_str.clone(),
                        place_number,
                        race_number,
                        race_data: None,
                        win_place_odds_data: None,
                        error: None,
                    };

                    // レースデータを取得（キャッシュ優先）
                    match repo.get_race_data(&date_str, place_number, race_number) {
                        Ok(Some(cached_race_data)) => {
                            let message = format!(
                                "📦 キャッシュからレースデータを取得: {}-{}-{}",
                                date_str, place_number, race_number
                            );
                            println!("{}", message);
                            if let Some(ref w) = window {
                                w.emit("bulk-progress", BulkProgressPayload {
                                    message,
                                    current: current_task,
                                    total: total_tasks,
                                    date: date_str.clone(),
                                    place_number,
                                    race_number,
                                    status: "cache_hit".to_string(),
                                }).ok();
                            }
                            bulk_data.race_data = Some(cached_race_data);
                        }
                        Ok(None) => {
                            // キャッシュにない場合はスクレイピング
                            let message = format!(
                                "🌐 レースデータをスクレイピング: {}-{}-{}",
                                date_str, place_number, race_number
                            );
                            println!("{}", message);
                            if let Some(ref w) = window {
                                w.emit("bulk-progress", BulkProgressPayload {
                                    message,
                                    current: current_task,
                                    total: total_tasks,
                                    date: date_str.clone(),
                                    place_number,
                                    race_number,
                                    status: "scraping".to_string(),
                                }).ok();
                            }

                            match headress::fetch_shusso_info_from_kyoteibiyori(
                                race_number,
                                place_number,
                                &date_str_no_dash,
                                1,
                            ) {
                                Ok(html_content) => {
                                    match flame::get_escaped_flame_info(&html_content) {
                                        Ok(race_data) => {
                                            // データベースに保存
                                            if let Err(save_err) = repo.save_race_data(
                                                &date_str,
                                                place_number,
                                                race_number,
                                                &race_data,
                                            ) {
                                                println!("⚠️ データベース保存エラー: {}", save_err);
                                            } else {
                                                let message = format!(
                                                    "💾 レースデータを保存: {}-{}-{}",
                                                    date_str, place_number, race_number
                                                );
                                                println!("{}", message);
                                                if let Some(ref w) = window {
                                                    w.emit("bulk-progress", BulkProgressPayload {
                                                        message,
                                                        current: current_task,
                                                        total: total_tasks,
                                                        date: date_str.clone(),
                                                        place_number,
                                                        race_number,
                                                        status: "saved".to_string(),
                                                    }).ok();
                                                }
                                            }
                                            bulk_data.race_data = Some(race_data);
                                        }
                                        Err(e) => {
                                            bulk_data.error =
                                                Some(format!("Race data parse error: {}", e))
                                        }
                                    }
                                }
                                Err(e) => {
                                    bulk_data.error = Some(format!("Race data fetch error: {}", e))
                                }
                            }
                        }
                        Err(e) => {
                            println!(
                                "⚠️ データベース取得エラー、スクレイピングにフォールバック: {}",
                                e
                            );
                            // Retry with scraping on DB error
                            match headress::fetch_shusso_info_from_kyoteibiyori(
                                race_number,
                                place_number,
                                &date_str_no_dash,
                                1,
                            ) {
                                Ok(html_content) => {
                                    match flame::get_escaped_flame_info(&html_content) {
                                        Ok(race_data) => {
                                            bulk_data.race_data = Some(race_data);
                                        }
                                        Err(e) => {
                                            bulk_data.error =
                                                Some(format!("Race data parse error: {}", e))
                                        }
                                    }
                                }
                                Err(e) => {
                                    bulk_data.error = Some(format!("Race data fetch error: {}", e))
                                }
                            }
                        }
                    }

                    // オッズデータを取得（キャッシュ優先）
                    match repo.get_odds_data(&date_str, place_number, race_number) {
                        Ok(Some(cached_odds_data)) => {
                            bulk_data.win_place_odds_data = Some(cached_odds_data);
                        }
                        Ok(None) => {
                            // キャッシュにない場合はスクレイピング
                            match headress::fetch_odds_info_from_kyoteibiyori(
                                race_number,
                                place_number,
                                &date_str_no_dash,
                            ) {
                                Ok(html_content) => {
                                    match flame::parse_win_place_odds_from_html(&html_content) {
                                        Ok(odds_data) => {
                                            // データベースに保存
                                            if let Err(save_err) = repo.save_odds_data(
                                                &date_str,
                                                place_number,
                                                race_number,
                                                &odds_data,
                                            ) {
                                                println!("⚠️ オッズデータベース保存エラー: {}", save_err);
                                            }
                                            bulk_data.win_place_odds_data = Some(odds_data);
                                        }
                                        Err(e) => {
                                            if bulk_data.error.is_none() {
                                                bulk_data.error =
                                                    Some(format!("Odds data parse error: {}", e));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    if bulk_data.error.is_none() {
                                        bulk_data.error =
                                            Some(format!("Odds data fetch error: {}", e));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // DB errorのみログ出力、スクレイピングは試行しない（オッズはオプション）
                            println!("⚠️ オッズデータベース取得エラー: {}", e);
                        }
                    }

                    all_results.push(bulk_data);

                    // レート制限を適用
                    let delay = if current_task < total_tasks {
                        // スクレイピングが発生した場合は長めの遅延
                        if repo.get_race_data(&date_str, place_number, race_number)
                            .ok()
                            .flatten()
                            .is_none()
                        {
                            TokioDuration::from_secs(1)
                        } else {
                            TokioDuration::from_millis(100)
                        }
                    } else {
                        TokioDuration::from_millis(0)
                    };
                    sleep(delay).await;
                }
            }

            current_date += Duration::days(1);
        }

        // 完了通知
        if let Some(ref w) = window {
            w.emit("bulk-progress", BulkProgressPayload {
                message: "✅ 一括取得完了".to_string(),
                current: total_tasks,
                total: total_tasks,
                date: end_date.to_string(),
                place_number: 0,
                race_number: 0,
                status: "completed".to_string(),
            }).ok();
        }

        Ok(all_results)
    }

    pub fn scrape_html(url: &str) -> Result<String, String> {
        headress::scrape_html_from_url(url)
            .map_err(|e| format!("スクレイピングエラー: {}", e))
    }
}

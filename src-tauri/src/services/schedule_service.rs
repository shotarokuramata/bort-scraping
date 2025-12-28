use crate::fetcher;
use crate::parse::official;
use crate::models::venue::{ActiveRace, RaceVenue, AllVenuesResponse, VenueStatus};
use std::collections::{HashMap, HashSet};

pub struct ScheduleService;

impl ScheduleService {
    pub async fn get_monthly_schedule() -> Result<official::MonthlySchedule, String> {
        let current_month = chrono::Local::now().format("%Y%m").to_string();
        let file_path = format!("bort-html/monthly_schedule_{}.html", current_month);

        // 1. 必要に応じてHTMLを取得
        if !std::fs::metadata(&file_path).is_ok() {
            fetcher::fetch_and_cache_monthly_schedule().await?;
        }

        // 2. ファイルを直接パース（ファイルI/Oは別スレッドで実行）
        tokio::task::spawn_blocking(move || {
            let html = std::fs::read_to_string(&file_path)
                .map_err(|e| format!("ファイル読み込みエラー: {}", e))?;

            official::parse_monthly_schedule(&html).map_err(|e| format!("パースエラー: {}", e))
        })
        .await
        .map_err(|e| format!("Task execution error: {}", e))?
    }

    pub async fn get_active_races() -> Result<ActiveRace, String> {
        // 月間スケジュールを取得してパース
        let monthly_schedule = Self::get_monthly_schedule().await?;

        // 今日開催中の競艇場を抽出
        let today = chrono::Local::now().date_naive();
        let today_str = today.format("%Y-%m-%d").to_string();

        let mut active_venues = Vec::new();

        for event in monthly_schedule.events {
            // イベントの開始日と終了日を計算
            let start_date = chrono::NaiveDate::parse_from_str(&event.start_date, "%Y-%m-%d")
                .map_err(|e| format!("日付パースエラー: {}", e))?;
            let end_date = start_date + chrono::Duration::days(event.duration_days as i64 - 1);

            // 今日がイベント期間内かチェック
            if today >= start_date && today <= end_date {
                // 既に追加済みの競艇場かチェック
                if !active_venues
                    .iter()
                    .any(|v: &RaceVenue| v.place_id == event.venue_id)
                {
                    active_venues.push(RaceVenue {
                        place_id: event.venue_id,
                        place_name: event.venue_name,
                        races: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], // 12レース固定
                    });
                }
            }
        }

        Ok(ActiveRace {
            date: today_str,
            venues: active_venues,
        })
    }

    pub async fn get_all_venues_with_status() -> Result<AllVenuesResponse, String> {
        // 今日開催中の競艇場を取得
        let active_races = Self::get_active_races().await?;
        let today = active_races.date;

        // 全競艇場マスターデータを取得
        let all_places = Self::get_all_venue_names();

        // 開催中の競艇場IDセットを作成
        let active_place_ids: HashSet<u32> =
            active_races.venues.iter().map(|v| v.place_id).collect();

        // 全競艇場のステータスを作成
        let venues: Vec<VenueStatus> = all_places
            .iter()
            .map(|(place_id, place_name)| {
                let is_active = active_place_ids.contains(place_id);
                let races = if is_active {
                    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] // 12レース固定
                } else {
                    vec![] // 非開催時は空
                };

                VenueStatus {
                    place_id: *place_id,
                    place_name: place_name.clone(),
                    is_active,
                    races,
                }
            })
            .collect();

        Ok(AllVenuesResponse {
            date: today,
            venues,
        })
    }

    pub fn get_all_venue_names() -> HashMap<u32, String> {
        [
            (1, "桐生".to_string()),
            (2, "戸田".to_string()),
            (3, "江戸川".to_string()),
            (4, "平和島".to_string()),
            (5, "多摩川".to_string()),
            (6, "浜名湖".to_string()),
            (7, "蒲郡".to_string()),
            (8, "常滑".to_string()),
            (9, "津".to_string()),
            (10, "三国".to_string()),
            (11, "びわこ".to_string()),
            (12, "住之江".to_string()),
            (13, "尼崎".to_string()),
            (14, "鳴門".to_string()),
            (15, "丸亀".to_string()),
            (16, "児島".to_string()),
            (17, "宮島".to_string()),
            (18, "徳山".to_string()),
            (19, "下関".to_string()),
            (20, "若松".to_string()),
            (21, "芦屋".to_string()),
            (22, "福岡".to_string()),
            (23, "唐津".to_string()),
            (24, "大村".to_string()),
        ]
        .into_iter()
        .collect()
    }
}

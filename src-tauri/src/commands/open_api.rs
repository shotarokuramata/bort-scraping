use crate::models::open_api::{
    ApiDataType, PayoutStats, RaceResult, SearchParams, RaceRecord, RaceParticipantRecord, DataSummaryRow,
    BulkFetchSummary,
};
use crate::services::open_api_service::OpenApiService;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

// サービスのグローバルステート
pub type OpenApiServiceState = Arc<Mutex<Option<OpenApiService>>>;

/// Open API サービスを初期化（デフォルトパスを使用）
#[tauri::command]
pub async fn init_open_api_service(
    state: State<'_, OpenApiServiceState>,
) -> Result<String, String> {
    println!("🚀 Initializing Open API service with default DB path");

    let service = OpenApiService::new(None).await?;
    let mut service_state = state.lock().await;
    *service_state = Some(service);

    Ok("Open API service initialized successfully".to_string())
}

/// Previews データを取得
#[tauri::command]
pub async fn fetch_previews_data(
    state: State<'_, OpenApiServiceState>,
    date: String,
) -> Result<String, String> {
    // 日付フォーマット検証
    if date.len() != 8 || !date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid date format. Expected YYYYMMDD".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.fetch_data(ApiDataType::Previews, &date).await
}

/// Results データを取得
#[tauri::command]
pub async fn fetch_results_data(
    state: State<'_, OpenApiServiceState>,
    date: String,
) -> Result<String, String> {
    // 日付フォーマット検証
    if date.len() != 8 || !date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid date format. Expected YYYYMMDD".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.fetch_data(ApiDataType::Results, &date).await
}

/// Programs データを取得
#[tauri::command]
pub async fn fetch_programs_data(
    state: State<'_, OpenApiServiceState>,
    date: String,
) -> Result<String, String> {
    // 日付フォーマット検証
    if date.len() != 8 || !date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid date format. Expected YYYYMMDD".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.fetch_data(ApiDataType::Programs, &date).await
}

/// Previews データをデータベースに保存
#[tauri::command]
pub async fn save_previews_to_db(
    state: State<'_, OpenApiServiceState>,
    date: String,
    json_data: String,
) -> Result<usize, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.save_previews_data(&date, &json_data).await
}

/// Results データをデータベースに保存
#[tauri::command]
pub async fn save_results_to_db(
    state: State<'_, OpenApiServiceState>,
    date: String,
    json_data: String,
) -> Result<usize, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.save_results_data(&date, &json_data).await
}

/// Programs データをデータベースに保存
#[tauri::command]
pub async fn save_programs_to_db(
    state: State<'_, OpenApiServiceState>,
    date: String,
    json_data: String,
) -> Result<usize, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.save_programs_data(&date, &json_data).await
}

/// CSV エクスポート
#[tauri::command]
pub async fn export_open_api_to_csv(
    state: State<'_, OpenApiServiceState>,
    output_path: String,
    data_type: Option<String>,
) -> Result<usize, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    // data_type 文字列を ApiDataType に変換
    let api_data_type = match data_type.as_deref() {
        Some("previews") => Some(ApiDataType::Previews),
        Some("results") => Some(ApiDataType::Results),
        Some("programs") => Some(ApiDataType::Programs),
        None => None,
        Some(other) => {
            return Err(format!(
                "Invalid data_type: '{}'. Expected 'previews', 'results', 'programs', or null",
                other
            ))
        }
    };

    service.export_to_csv(&output_path, api_data_type).await
}

/// V3: CSVエクスポート（正規化スキーマ版）
///
/// races.csv と race_participants.csv の2ファイルを出力。
/// JSONカラムは除外され、すべてのカラムが展開された形式でエクスポートされる。
///
/// # Arguments
/// * `output_dir` - 出力先ディレクトリパス（例: "data/exports"）
///
/// # Returns
/// * `Ok((race_count, participant_count))` - エクスポートされたレース数と参加者数のタプル
#[tauri::command]
pub async fn export_open_api_to_csv_v3(
    state: State<'_, OpenApiServiceState>,
    output_dir: String,
) -> Result<(usize, usize), String> {
    // 出力ディレクトリ検証
    let path = std::path::Path::new(&output_dir);
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }
    if !path.is_dir() {
        return Err(format!("Output path is not a directory: {}", output_dir));
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.export_to_csv_v3(&output_dir).await
}

// ===== 高配当検索機能 =====

/// 高配当レース検索
#[tauri::command]
pub async fn search_high_payout_races(
    state: State<'_, OpenApiServiceState>,
    min_payout: i32,
    payout_type: String,
    limit: Option<i32>,
) -> Result<Vec<RaceResult>, String> {
    // payout_type バリデーション
    if !["win", "trifecta", "exacta", "place"].contains(&payout_type.as_str()) {
        return Err(format!(
            "Invalid payout_type: '{}'. Expected 'win', 'trifecta', 'exacta', or 'place'",
            payout_type
        ));
    }

    // min_payout バリデーション
    if min_payout < 0 {
        return Err("min_payout must be non-negative".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_high_payout_races(min_payout, payout_type, limit).await
}

/// 配当統計情報取得
#[tauri::command]
pub async fn get_payout_statistics(
    state: State<'_, OpenApiServiceState>,
) -> Result<PayoutStats, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.get_payout_statistics().await
}

// ===== V3検索API: 複合条件検索 =====

/// 複合条件検索
#[tauri::command]
pub async fn search_races_advanced(
    state: State<'_, OpenApiServiceState>,
    params: SearchParams,
) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_races_advanced(params).await
}

/// 選手番号での検索
#[tauri::command]
pub async fn search_races_by_racer(
    state: State<'_, OpenApiServiceState>,
    racer_number: i32,
    limit: Option<i32>,
) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
    if racer_number < 0 {
        return Err("racer_number must be non-negative".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_races_by_racer(racer_number, limit).await
}

/// 選手名での検索（部分一致）
#[tauri::command]
pub async fn search_races_by_racer_name(
    state: State<'_, OpenApiServiceState>,
    racer_name: String,
    limit: Option<i32>,
) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
    if racer_name.is_empty() {
        return Err("racer_name cannot be empty".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_races_by_racer_name(racer_name, limit).await
}

/// 級別での検索
#[tauri::command]
pub async fn search_races_by_class(
    state: State<'_, OpenApiServiceState>,
    racer_class: i32,
    limit: Option<i32>,
) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
    // 級別バリデーション: 1=A1, 2=A2, 3=B1, 4=B2
    if !(1..=4).contains(&racer_class) {
        return Err("racer_class must be between 1 and 4 (1=A1, 2=A2, 3=B1, 4=B2)".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_races_by_class(racer_class, limit).await
}

/// 日付範囲での検索
#[tauri::command]
pub async fn search_races_by_date_range(
    state: State<'_, OpenApiServiceState>,
    date_from: String,
    date_to: String,
    limit: Option<i32>,
) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
    // 日付フォーマット検証
    if date_from.len() != 8 || !date_from.chars().all(|c| c.is_numeric()) {
        return Err("Invalid date_from format. Expected YYYYMMDD".to_string());
    }
    if date_to.len() != 8 || !date_to.chars().all(|c| c.is_numeric()) {
        return Err("Invalid date_to format. Expected YYYYMMDD".to_string());
    }
    if date_from > date_to {
        return Err("date_from must be less than or equal to date_to".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_races_by_date_range(date_from, date_to, limit).await
}

/// 会場での検索
#[tauri::command]
pub async fn search_races_by_venue(
    state: State<'_, OpenApiServiceState>,
    venue_code: String,
    limit: Option<i32>,
) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
    // 会場コードバリデーション（01-24の2桁形式）
    if venue_code.len() != 2 || !venue_code.chars().all(|c| c.is_numeric()) {
        return Err("Invalid venue_code format. Expected 2-digit code (01-24)".to_string());
    }
    let venue_num: i32 = venue_code.parse().unwrap_or(0);
    if !(1..=24).contains(&venue_num) {
        return Err("venue_code must be between 01 and 24".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.search_races_by_venue(venue_code, limit).await
}

/// 日付ごとのデータ取得状況サマリーを取得
#[tauri::command]
pub async fn get_open_api_data_summary(
    state: State<'_, OpenApiServiceState>,
) -> Result<Vec<DataSummaryRow>, String> {
    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service.get_data_summary().await
}

// ===== Bulk Fetch Commands =====

/// Previews データの期間一括取得
#[tauri::command]
pub async fn fetch_previews_data_bulk(
    window: tauri::Window,
    state: State<'_, OpenApiServiceState>,
    start_date: String,  // YYYYMMDD形式
    end_date: String,    // YYYYMMDD形式
) -> Result<BulkFetchSummary, String> {
    // パラメータ検証
    if start_date.len() != 8 || !start_date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid start_date format. Expected YYYYMMDD".to_string());
    }
    if end_date.len() != 8 || !end_date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid end_date format. Expected YYYYMMDD".to_string());
    }
    if start_date > end_date {
        return Err("start_date must be less than or equal to end_date".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service
        .fetch_data_bulk(
            Some(window),
            ApiDataType::Previews,
            &start_date,
            &end_date,
        )
        .await
}

/// Results データの期間一括取得
#[tauri::command]
pub async fn fetch_results_data_bulk(
    window: tauri::Window,
    state: State<'_, OpenApiServiceState>,
    start_date: String,  // YYYYMMDD形式
    end_date: String,    // YYYYMMDD形式
) -> Result<BulkFetchSummary, String> {
    // パラメータ検証
    if start_date.len() != 8 || !start_date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid start_date format. Expected YYYYMMDD".to_string());
    }
    if end_date.len() != 8 || !end_date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid end_date format. Expected YYYYMMDD".to_string());
    }
    if start_date > end_date {
        return Err("start_date must be less than or equal to end_date".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service
        .fetch_data_bulk(
            Some(window),
            ApiDataType::Results,
            &start_date,
            &end_date,
        )
        .await
}

/// Programs データの期間一括取得
#[tauri::command]
pub async fn fetch_programs_data_bulk(
    window: tauri::Window,
    state: State<'_, OpenApiServiceState>,
    start_date: String,  // YYYYMMDD形式
    end_date: String,    // YYYYMMDD形式
) -> Result<BulkFetchSummary, String> {
    // パラメータ検証
    if start_date.len() != 8 || !start_date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid start_date format. Expected YYYYMMDD".to_string());
    }
    if end_date.len() != 8 || !end_date.chars().all(|c| c.is_numeric()) {
        return Err("Invalid end_date format. Expected YYYYMMDD".to_string());
    }
    if start_date > end_date {
        return Err("start_date must be less than or equal to end_date".to_string());
    }

    let service_state = state.lock().await;
    let service = service_state
        .as_ref()
        .ok_or("Service not initialized. Call init_open_api_service first.")?;

    service
        .fetch_data_bulk(
            Some(window),
            ApiDataType::Programs,
            &start_date,
            &end_date,
        )
        .await
}

use crate::models::open_api::{ApiDataType, PayoutStats, RaceResult};
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

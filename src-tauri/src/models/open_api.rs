use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===== API レスポンス型 =====

// 1. Previews（レース予測情報）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewsResponse {
    pub previews: Vec<RacePreview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacePreview {
    pub race_date: String,
    pub race_stadium_number: i32,
    pub race_number: i32,
    pub race_wind: Option<f64>,
    pub race_wind_direction_number: Option<f64>,
    pub race_wave: Option<f64>,
    pub race_weather_number: Option<f64>,
    pub race_temperature: Option<f64>,
    pub race_water_temperature: Option<f64>,
    pub boats: HashMap<String, PreviewRacerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRacerInfo {
    pub racer_boat_number: Option<i32>,
    pub racer_course_number: Option<i32>,
    pub racer_start_timing: Option<f64>,
    pub racer_weight: Option<f64>,
    pub racer_weight_adjustment: Option<f64>,
    pub racer_exhibition_time: Option<f64>,
    pub racer_tilt_adjustment: Option<f64>,
}

// 2. Results（レース結果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultsResponse {
    pub results: Vec<RaceResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceResult {
    pub race_date: String,
    pub race_stadium_number: i32,
    pub race_number: i32,
    pub race_wind: Option<f64>,
    pub race_wind_direction_number: Option<f64>,
    pub race_wave: Option<f64>,
    pub race_weather_number: Option<f64>,
    pub race_temperature: Option<f64>,
    pub race_water_temperature: Option<f64>,
    pub race_technique_number: Option<f64>,
    pub boats: Vec<ResultRacerInfo>,
    pub payouts: PayoutInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultRacerInfo {
    pub racer_boat_number: i32,
    pub racer_course_number: Option<i32>,
    pub racer_start_timing: Option<f64>,
    pub racer_place_number: Option<i32>,
    pub racer_number: Option<i32>,
    pub racer_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutInfo {
    pub win: Option<Vec<PayoutEntry>>,
    pub place: Option<Vec<PayoutEntry>>,
    pub exacta: Option<Vec<PayoutEntry>>,
    pub quinella: Option<Vec<PayoutEntry>>,
    pub quinella_place: Option<Vec<PayoutEntry>>,
    pub trifecta: Option<Vec<PayoutEntry>>,
    pub trio: Option<Vec<PayoutEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutEntry {
    pub combination: Option<String>,
    pub payout: Option<i32>,
}

// 3. Programs（出走表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramsResponse {
    pub programs: Vec<RaceProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceProgram {
    pub race_date: String,
    pub race_stadium_number: i32,
    pub race_number: i32,
    pub race_closed_at: Option<String>,
    pub race_grade_number: Option<i32>,
    pub race_title: Option<String>,
    pub race_subtitle: Option<String>,
    pub race_distance: Option<i32>,
    pub boats: Vec<ProgramRacerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramRacerInfo {
    pub racer_boat_number: Option<i32>,
    pub racer_name: Option<String>,
    pub racer_number: Option<i32>,
    pub racer_class_number: Option<i32>,
    pub racer_branch_number: Option<i32>,         // 支部番号（追加）
    pub racer_birthplace_number: Option<i32>,     // 出身地番号（追加）
    pub racer_age: Option<i32>,
    pub racer_weight: Option<f64>,
    pub racer_flying_count: Option<i32>,
    pub racer_late_count: Option<i32>,
    pub racer_average_start_timing: Option<f64>,
    pub racer_national_top_1_percent: Option<f64>,
    pub racer_national_top_2_percent: Option<f64>,
    pub racer_national_top_3_percent: Option<f64>,
    pub racer_local_top_1_percent: Option<f64>,
    pub racer_local_top_2_percent: Option<f64>,
    pub racer_local_top_3_percent: Option<f64>,
    pub racer_assigned_motor_number: Option<i32>,
    pub racer_assigned_motor_top_2_percent: Option<f64>,  // モーター2連対率（追加）
    pub racer_assigned_motor_top_3_percent: Option<f64>,  // モーター3連対率（追加）
    pub racer_assigned_boat_number: Option<i32>,
    pub racer_assigned_boat_top_2_percent: Option<f64>,   // ボート2連対率（追加）
    pub racer_assigned_boat_top_3_percent: Option<f64>,   // ボート3連対率（追加）
}

// ===== データベース保存用レコード型 =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreviewRecord {
    pub id: i64,
    pub date: String,
    pub venue_code: String,
    pub race_number: i32,
    pub data_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResultRecord {
    pub id: i64,
    pub date: String,
    pub venue_code: String,
    pub race_number: i32,
    pub data_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProgramRecord {
    pub id: i64,
    pub date: String,
    pub venue_code: String,
    pub race_number: i32,
    pub data_json: String,
    pub created_at: String,
    pub updated_at: String,
}

// ===== CSV 出力用型 =====

#[derive(Debug, Clone, Serialize)]
pub struct CsvExportRow {
    pub date: String,
    pub venue_code: String,
    pub race_number: i32,
    pub data_type: String,
    pub data_json: String,
}

// ===== Enum型 =====

#[derive(Debug, Clone, Copy)]
pub enum ApiDataType {
    Previews,
    Results,
    Programs,
}

impl ApiDataType {
    pub fn as_str(&self) -> &str {
        match self {
            ApiDataType::Previews => "previews",
            ApiDataType::Results => "results",
            ApiDataType::Programs => "programs",
        }
    }
}

// ===== 配当統計情報用構造体 =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PayoutStats {
    pub avg_trifecta: Option<f64>,
    pub max_trifecta: Option<i32>,
    pub avg_win: Option<f64>,
    pub max_win: Option<i32>,
}

// ===== 検索パラメータ構造体 =====

/// 複合条件検索のパラメータ
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchParams {
    // 選手条件
    pub racer_number: Option<i32>,
    pub racer_name: Option<String>,
    pub racer_class: Option<i32>,  // 1=A1, 2=A2, 3=B1, 4=B2

    // 日付・会場条件
    pub date_from: Option<String>,  // YYYYMMDD
    pub date_to: Option<String>,    // YYYYMMDD
    pub venue_code: Option<String>,

    // レース条件
    pub race_grade: Option<i32>,  // 1=SG, 2=G1, 3=G2, 4=G3, 5=一般
    pub race_number: Option<i32>,

    // 配当条件
    pub min_trifecta_payout: Option<i32>,
    pub max_trifecta_payout: Option<i32>,
    pub min_win_payout: Option<i32>,

    // 気象条件
    pub min_wind: Option<f64>,
    pub max_wind: Option<f64>,
    pub min_wave: Option<f64>,
    pub max_wave: Option<f64>,
    pub min_temperature: Option<f64>,
    pub max_temperature: Option<f64>,

    // 勝者条件
    pub winner_boat_number: Option<i32>,

    // 着順条件（選手検索時）
    pub place_number: Option<i32>,

    // 結果数制限
    pub limit: Option<i32>,
}

// ===== V3マイグレーション用構造体（正規化テーブル） =====

/// racesテーブルのレコード構造体
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RaceRecord {
    pub id: i64,
    pub race_date: String,
    pub venue_code: String,
    pub race_number: i32,
    // 気象条件
    pub race_wind: Option<f64>,
    pub race_wind_direction_number: Option<f64>,
    pub race_wave: Option<f64>,
    pub race_weather_number: Option<f64>,
    pub race_temperature: Option<f64>,
    pub race_water_temperature: Option<f64>,
    pub race_technique_number: Option<f64>,
    // 配当情報
    pub win_payout: Option<i32>,
    pub place_payout_max: Option<i32>,
    pub exacta_payout: Option<i32>,
    pub quinella_payout: Option<i32>,
    pub trifecta_payout: Option<i32>,
    pub trio_payout: Option<i32>,
    // 勝者情報
    pub winner_boat_number: Option<i32>,
    pub winner_racer_number: Option<i32>,
    // レース詳細（Programs APIから）
    pub race_grade_number: Option<i32>,
    pub race_title: Option<String>,
    pub race_subtitle: Option<String>,
    pub race_distance: Option<i32>,
    // 生JSONデータ
    pub result_data_json: Option<String>,
    pub program_data_json: Option<String>,
    // メタデータ
    pub created_at: String,
    pub updated_at: String,
}

/// race_participantsテーブルのレコード構造体
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RaceParticipantRecord {
    pub id: i64,
    pub race_id: i64,
    // 艇・選手情報
    pub boat_number: i32,
    pub racer_number: Option<i32>,
    pub racer_name: Option<String>,
    // 級別・所属（Programs APIから）
    pub racer_class_number: Option<i32>,
    pub racer_branch_number: Option<i32>,
    pub racer_birthplace_number: Option<i32>,
    pub racer_age: Option<i32>,
    pub racer_weight: Option<f64>,
    // スタート・進入情報
    pub course_number: Option<i32>,
    pub start_timing: Option<f64>,
    pub entry_number: Option<i32>,
    // レース結果
    pub place_number: Option<i32>,
    pub decision_hand: Option<String>,
    // 成績情報（Programs APIから）
    pub flying_count: Option<i32>,
    pub late_count: Option<i32>,
    pub average_start_timing: Option<f64>,
    pub national_top_1_percent: Option<f64>,
    pub national_top_2_percent: Option<f64>,
    pub national_top_3_percent: Option<f64>,
    pub local_top_1_percent: Option<f64>,
    pub local_top_2_percent: Option<f64>,
    pub local_top_3_percent: Option<f64>,
    // モーター・ボート情報（Programs APIから）
    pub assigned_motor_number: Option<i32>,
    pub assigned_motor_top_2_percent: Option<f64>,
    pub assigned_motor_top_3_percent: Option<f64>,
    pub assigned_boat_number: Option<i32>,
    pub assigned_boat_top_2_percent: Option<f64>,
    pub assigned_boat_top_3_percent: Option<f64>,
    // メタデータ
    pub created_at: String,
    pub updated_at: String,
}

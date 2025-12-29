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
    pub racer_assigned_boat_number: Option<i32>,
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

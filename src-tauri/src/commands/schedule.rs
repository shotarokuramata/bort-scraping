use crate::services::schedule_service::ScheduleService;
use crate::parse::official::MonthlySchedule;
use crate::models::venue::{ActiveRace, AllVenuesResponse};

#[tauri::command]
pub async fn get_monthly_schedule() -> Result<MonthlySchedule, String> {
    ScheduleService::get_monthly_schedule().await
}

#[tauri::command]
pub async fn get_active_races() -> Result<ActiveRace, String> {
    ScheduleService::get_active_races().await
}

#[tauri::command]
pub async fn get_all_venues_with_status() -> Result<AllVenuesResponse, String> {
    ScheduleService::get_all_venues_with_status().await
}

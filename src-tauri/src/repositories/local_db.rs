use norimaki_db::{FileStore, KeyValueStore, serialize_to_string, deserialize_from_string, Result};
use crate::models::race::{RaceData, OddsData};
use std::sync::Mutex;

static DB: Mutex<Option<FileStore>> = Mutex::new(None);

const DB_FILE_PATH: &str = "bort_race_data.json";

fn get_db() -> Result<&'static Mutex<Option<FileStore>>> {
    let mut db = DB.lock().unwrap();
    if db.is_none() {
        *db = Some(FileStore::new(DB_FILE_PATH)?);
    }
    Ok(&DB)
}

/// Local database repository using norimaki-db
pub struct LocalDbRepository;

impl LocalDbRepository {
    pub fn new() -> Result<Self> {
        // Initialize DB
        get_db()?;
        Ok(LocalDbRepository)
    }

    pub fn save_race_data(
        &self,
        date: &str,
        place_number: u32,
        race_number: u32,
        race_data: &RaceData,
    ) -> Result<()> {
        let db_lock = get_db()?;
        let mut db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_mut().unwrap();

        let key = format!("race_{}_{}_{}_{}", date, place_number, race_number, "data");
        let value = serialize_to_string(race_data)?;

        db.put(key, value)
    }

    pub fn get_race_data(
        &self,
        date: &str,
        place_number: u32,
        race_number: u32,
    ) -> Result<Option<RaceData>> {
        let db_lock = get_db()?;
        let db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_ref().unwrap();

        let key = format!("race_{}_{}_{}_{}", date, place_number, race_number, "data");

        match db.get(&key)? {
            Some(value) => {
                let race_data: RaceData = deserialize_from_string(&value)?;
                Ok(Some(race_data))
            }
            None => Ok(None)
        }
    }

    pub fn save_odds_data(
        &self,
        date: &str,
        place_number: u32,
        race_number: u32,
        odds_data: &OddsData,
    ) -> Result<()> {
        let db_lock = get_db()?;
        let mut db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_mut().unwrap();

        let key = format!("race_{}_{}_{}_{}", date, place_number, race_number, "odds");
        let value = serialize_to_string(odds_data)?;

        db.put(key, value)
    }

    pub fn get_odds_data(
        &self,
        date: &str,
        place_number: u32,
        race_number: u32,
    ) -> Result<Option<OddsData>> {
        let db_lock = get_db()?;
        let db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_ref().unwrap();

        let key = format!("race_{}_{}_{}_{}", date, place_number, race_number, "odds");

        match db.get(&key)? {
            Some(value) => {
                let odds_data: OddsData = deserialize_from_string(&value)?;
                Ok(Some(odds_data))
            }
            None => Ok(None)
        }
    }


    pub fn get_all_race_keys(&self) -> Result<Vec<String>> {
        let db_lock = get_db()?;
        let db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_ref().unwrap();

        let all_keys = db.keys()?;
        let race_keys: Vec<String> = all_keys
            .into_iter()
            .filter(|key| key.contains("race_") && (key.ends_with("_data") || key.ends_with("_odds")))
            .collect();

        Ok(race_keys)
    }

    pub fn delete_race_data(
        &self,
        date: &str,
        place_number: u32,
        race_number: u32,
    ) -> Result<()> {
        let db_lock = get_db()?;
        let mut db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_mut().unwrap();

        let data_key = format!("race_{}_{}_{}_{}", date, place_number, race_number, "data");
        let odds_key = format!("race_{}_{}_{}_{}", date, place_number, race_number, "odds");

        // データが存在しなくてもエラーにしない
        let _ = db.delete(&data_key);
        let _ = db.delete(&odds_key);

        Ok(())
    }

    pub fn clear_all_data(&self) -> Result<()> {
        let db_lock = get_db()?;
        let mut db_guard = db_lock.lock().unwrap();
        let db = db_guard.as_mut().unwrap();

        db.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::race::*;

    fn create_sample_race_data() -> RaceData {
        RaceData {
            escape_last_year: 50.5,
            escape_last_half_year: 48.2,
            allow_escape_last_year: 30.1,
            allow_escape_last_half_year: 32.4,
            pierce_last_year: 15.3,
            pierce_last_half_year: 16.8,
            overtake_last_year: 20.2,
            overtake_last_half_year: 18.9,
            player_basic_info: PlayerBasicInfo {
                registration_number: "4444".to_string(),
                name: "テスト選手".to_string(),
                class_level: "A1".to_string(),
                period: "130".to_string(),
                support_group: "群馬".to_string(),
                gender: "男".to_string(),
            },
            detailed_performance: DetailedPerformanceData {
                first_place_rate: PerformanceData {
                    this_period: Some(0.25),
                    last_6_months: Some(0.30),
                    last_3_months: Some(0.28),
                    last_1_month: Some(0.35),
                    local_venue: Some(0.32),
                    general_races: Some(0.27),
                    sg_g1: Some(0.20),
                },
                lane_win_rate: LaneWinRateData {
                    last_1_year: Some(0.18),
                    last_6_months: Some(0.20),
                },
            },
            st_data: STRelatedData {
                average_st: STData {
                    this_period: Some(0.15),
                    last_6_months: Some(0.14),
                    last_3_months: Some(0.16),
                    last_1_month: Some(0.13),
                    local_venue: Some(0.15),
                    general_races: Some(0.15),
                    sg_g1: Some(0.17),
                    first_day: Some(0.16),
                    final_day: Some(0.14),
                    night_races: Some(0.15),
                    flying_history: Some(0.02),
                },
                st_ranking: STData {
                    this_period: Some(3.2),
                    last_6_months: Some(3.1),
                    last_3_months: Some(3.3),
                    last_1_month: Some(2.9),
                    local_venue: Some(3.0),
                    general_races: Some(3.2),
                    sg_g1: Some(3.8),
                    first_day: Some(3.1),
                    final_day: Some(3.0),
                    night_races: Some(3.2),
                    flying_history: None,
                },
                st_analysis: STAnalysisData {
                    stability_rate: Some(0.85),
                    break_out_rate: Some(0.03),
                    late_start_rate: Some(0.12),
                },
            },
            winning_hand: WinningHandData {
                escape_rate_6months: Some(0.45),
                let_escape_rate_6months: Some(0.30),
                pierced_rate_6months: Some(0.15),
                pierce_rate_6months: Some(0.20),
                overtake_rate_6months: Some(0.18),
            },
        }
    }

    fn create_sample_odds_data() -> OddsData {
        OddsData {
            betting_type: BettingType::WinPlace,
            combinations: vec![
                OddsCombination {
                    first: 1,
                    second: 0,
                    third: None,
                    odds: 2.5,
                    is_combined: false,
                    range_text: None,
                },
                OddsCombination {
                    first: 2,
                    second: 0,
                    third: None,
                    odds: 3.2,
                    is_combined: false,
                    range_text: None,
                },
            ],
        }
    }

    #[test]
    fn test_save_and_get_race_data() {
        let repo = LocalDbRepository::new().unwrap();
        let race_data = create_sample_race_data();

        let save_result = repo.save_race_data("2025-09-15", 1, 1, &race_data);
        assert!(save_result.is_ok(), "Failed to save race data: {:?}", save_result.err());

        let get_result = repo.get_race_data("2025-09-15", 1, 1);
        assert!(get_result.is_ok(), "Failed to get race data: {:?}", get_result.err());

        let retrieved_data = get_result.unwrap();
        assert!(retrieved_data.is_some(), "No data retrieved");

        let retrieved_race_data = retrieved_data.unwrap();
        assert_eq!(retrieved_race_data.player_basic_info.name, "テスト選手");
        assert_eq!(retrieved_race_data.escape_last_year, 50.5);

        println!("✅ Race data save/get test passed");
    }

    #[test]
    fn test_save_and_get_odds_data() {
        let repo = LocalDbRepository::new().unwrap();
        let odds_data = create_sample_odds_data();

        let save_result = repo.save_odds_data("2025-09-15", 1, 1, &odds_data);
        assert!(save_result.is_ok(), "Failed to save odds data: {:?}", save_result.err());

        let get_result = repo.get_odds_data("2025-09-15", 1, 1);
        assert!(get_result.is_ok(), "Failed to get odds data: {:?}", get_result.err());

        let retrieved_data = get_result.unwrap();
        assert!(retrieved_data.is_some(), "No odds data retrieved");

        let retrieved_odds_data = retrieved_data.unwrap();
        assert_eq!(retrieved_odds_data.betting_type, BettingType::WinPlace);
        assert_eq!(retrieved_odds_data.combinations.len(), 2);
        assert_eq!(retrieved_odds_data.combinations[0].odds, 2.5);

        println!("✅ Odds data save/get test passed");
    }

    #[test]
    fn test_get_all_race_keys() {
        let repo = LocalDbRepository::new().unwrap();
        let race_data = create_sample_race_data();
        let odds_data = create_sample_odds_data();

        let _ = repo.save_race_data("2025-09-15", 1, 1, &race_data);
        let _ = repo.save_race_data("2025-09-15", 1, 2, &race_data);
        let _ = repo.save_odds_data("2025-09-15", 1, 1, &odds_data);

        let keys_result = repo.get_all_race_keys();
        assert!(keys_result.is_ok(), "Failed to get race keys: {:?}", keys_result.err());

        let keys = keys_result.unwrap();
        assert!(keys.len() >= 3, "Expected at least 3 keys, got {}", keys.len());

        let has_data_key = keys.iter().any(|k| k.contains("race_2025-09-15_1_1_data"));
        let has_odds_key = keys.iter().any(|k| k.contains("race_2025-09-15_1_1_odds"));

        assert!(has_data_key, "Race data key not found in: {:?}", keys);
        assert!(has_odds_key, "Odds data key not found in: {:?}", keys);

        println!("✅ Get all race keys test passed. Found {} keys", keys.len());
    }

    #[test]
    fn test_delete_race_data() {
        let repo = LocalDbRepository::new().unwrap();
        let race_data = create_sample_race_data();
        let odds_data = create_sample_odds_data();

        let _ = repo.save_race_data("2025-09-15", 2, 1, &race_data);
        let _ = repo.save_odds_data("2025-09-15", 2, 1, &odds_data);

        let get_result = repo.get_race_data("2025-09-15", 2, 1);
        assert!(get_result.unwrap().is_some(), "Data should exist before deletion");

        let delete_result = repo.delete_race_data("2025-09-15", 2, 1);
        assert!(delete_result.is_ok(), "Failed to delete race data: {:?}", delete_result.err());

        let get_after_delete = repo.get_race_data("2025-09-15", 2, 1);
        assert!(get_after_delete.unwrap().is_none(), "Data should be deleted");

        let get_odds_after_delete = repo.get_odds_data("2025-09-15", 2, 1);
        assert!(get_odds_after_delete.unwrap().is_none(), "Odds data should be deleted");

        println!("✅ Delete race data test passed");
    }
}

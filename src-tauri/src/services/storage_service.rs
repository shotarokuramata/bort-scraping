use crate::repositories::local_db::LocalDbRepository;
use crate::models::race::{RaceData, OddsData};

pub struct StorageService {
    repo: LocalDbRepository,
}

impl StorageService {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            repo: LocalDbRepository::new().map_err(|e| e.to_string())?,
        })
    }

    pub fn save_race(&self, date: &str, place: u32, race: u32, data: &RaceData) -> Result<(), String> {
        self.repo
            .save_race_data(date, place, race, data)
            .map_err(|e| e.to_string())
    }

    pub fn get_race(&self, date: &str, place: u32, race: u32) -> Result<Option<RaceData>, String> {
        self.repo
            .get_race_data(date, place, race)
            .map_err(|e| e.to_string())
    }

    pub fn save_odds(&self, date: &str, place: u32, race: u32, data: &OddsData) -> Result<(), String> {
        self.repo
            .save_odds_data(date, place, race, data)
            .map_err(|e| e.to_string())
    }

    pub fn get_odds(&self, date: &str, place: u32, race: u32) -> Result<Option<OddsData>, String> {
        self.repo
            .get_odds_data(date, place, race)
            .map_err(|e| e.to_string())
    }

    pub fn get_all_race_keys(&self) -> Result<Vec<String>, String> {
        self.repo
            .get_all_race_keys()
            .map_err(|e| e.to_string())
    }

    pub fn delete_race(&self, date: &str, place: u32, race: u32) -> Result<(), String> {
        self.repo
            .delete_race_data(date, place, race)
            .map_err(|e| e.to_string())
    }

    pub fn clear_all(&self) -> Result<(), String> {
        self.repo
            .clear_all_data()
            .map_err(|e| e.to_string())
    }
}

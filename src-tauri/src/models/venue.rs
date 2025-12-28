#[derive(serde::Serialize)]
pub struct RaceVenue {
    pub place_id: u32,
    pub place_name: String,
    pub races: Vec<u32>, // 開催レース番号のリスト
}

#[derive(serde::Serialize)]
pub struct ActiveRace {
    pub date: String,
    pub venues: Vec<RaceVenue>,
}

#[derive(serde::Serialize)]
pub struct VenueStatus {
    pub place_id: u32,
    pub place_name: String,
    pub is_active: bool, // 開催中: true, 非開催: false
    pub races: Vec<u32>, // 開催中なら1-12、非開催なら空
}

#[derive(serde::Serialize)]
pub struct AllVenuesResponse {
    pub date: String,
    pub venues: Vec<VenueStatus>,
}

#[derive(serde::Serialize, Clone)]
pub struct BulkProgressPayload {
    pub message: String,
    pub current: usize,
    pub total: usize,
    pub date: String,
    pub place_number: u32,
    pub race_number: u32,
    pub status: String, // "cache_hit" | "scraping" | "saved" | "error"
}

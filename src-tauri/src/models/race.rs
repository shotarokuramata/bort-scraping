use std::fmt;

// ===== Player Information =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct PlayerBasicInfo {
    pub registration_number: String,
    pub name: String,
    pub class_level: String,
    pub period: String,
    pub support_group: String,
    pub gender: String,
}

impl Default for PlayerBasicInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerBasicInfo {
    pub fn new() -> Self {
        PlayerBasicInfo {
            registration_number: String::new(),
            name: String::new(),
            class_level: String::new(),
            period: String::new(),
            support_group: String::new(),
            gender: String::new(),
        }
    }
}

// ===== Performance Data =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct PerformanceData {
    pub this_period: Option<f64>,
    pub last_6_months: Option<f64>,
    pub last_3_months: Option<f64>,
    pub last_1_month: Option<f64>,
    pub local_venue: Option<f64>,
    pub general_races: Option<f64>,
    pub sg_g1: Option<f64>,
}

impl Default for PerformanceData {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceData {
    pub fn new() -> Self {
        PerformanceData {
            this_period: None,
            last_6_months: None,
            last_3_months: None,
            last_1_month: None,
            local_venue: None,
            general_races: None,
            sg_g1: None,
        }
    }
}

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct LaneWinRateData {
    pub last_1_year: Option<f64>,
    pub last_6_months: Option<f64>,
}

impl Default for LaneWinRateData {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneWinRateData {
    pub fn new() -> Self {
        LaneWinRateData {
            last_1_year: None,
            last_6_months: None,
        }
    }
}

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct DetailedPerformanceData {
    pub first_place_rate: PerformanceData,
    pub lane_win_rate: LaneWinRateData,
}

impl Default for DetailedPerformanceData {
    fn default() -> Self {
        Self::new()
    }
}

impl DetailedPerformanceData {
    pub fn new() -> Self {
        DetailedPerformanceData {
            first_place_rate: PerformanceData::new(),
            lane_win_rate: LaneWinRateData::new(),
        }
    }
}

// ===== ST (Start Timing) Data =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct STData {
    pub this_period: Option<f64>,
    pub last_6_months: Option<f64>,
    pub last_3_months: Option<f64>,
    pub last_1_month: Option<f64>,
    pub local_venue: Option<f64>,
    pub general_races: Option<f64>,
    pub sg_g1: Option<f64>,
    pub first_day: Option<f64>,
    pub final_day: Option<f64>,
    pub night_races: Option<f64>,
    pub flying_history: Option<f64>,
}

impl Default for STData {
    fn default() -> Self {
        Self::new()
    }
}

impl STData {
    pub fn new() -> Self {
        STData {
            this_period: None,
            last_6_months: None,
            last_3_months: None,
            last_1_month: None,
            local_venue: None,
            general_races: None,
            sg_g1: None,
            first_day: None,
            final_day: None,
            night_races: None,
            flying_history: None,
        }
    }
}

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct STAnalysisData {
    pub stability_rate: Option<f64>,
    pub break_out_rate: Option<f64>,
    pub late_start_rate: Option<f64>,
}

impl Default for STAnalysisData {
    fn default() -> Self {
        Self::new()
    }
}

impl STAnalysisData {
    pub fn new() -> Self {
        STAnalysisData {
            stability_rate: None,
            break_out_rate: None,
            late_start_rate: None,
        }
    }
}

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct STRelatedData {
    pub average_st: STData,
    pub st_ranking: STData,
    pub st_analysis: STAnalysisData,
}

impl Default for STRelatedData {
    fn default() -> Self {
        Self::new()
    }
}

impl STRelatedData {
    pub fn new() -> Self {
        STRelatedData {
            average_st: STData::new(),
            st_ranking: STData::new(),
            st_analysis: STAnalysisData::new(),
        }
    }
}

// ===== Winning Hand Data =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct WinningHandData {
    pub escape_rate_6months: Option<f64>,
    pub let_escape_rate_6months: Option<f64>,
    pub pierced_rate_6months: Option<f64>,
    pub pierce_rate_6months: Option<f64>,
    pub overtake_rate_6months: Option<f64>,
}

impl Default for WinningHandData {
    fn default() -> Self {
        Self::new()
    }
}

impl WinningHandData {
    pub fn new() -> Self {
        WinningHandData {
            escape_rate_6months: None,
            let_escape_rate_6months: None,
            pierced_rate_6months: None,
            pierce_rate_6months: None,
            overtake_rate_6months: None,
        }
    }
}

// ===== Race Data =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct RaceData {
    pub escape_last_year: f64,
    pub escape_last_half_year: f64,
    pub allow_escape_last_year: f64,
    pub allow_escape_last_half_year: f64,
    pub pierce_last_year: f64,
    pub pierce_last_half_year: f64,
    pub overtake_last_year: f64,
    pub overtake_last_half_year: f64,
    pub player_basic_info: PlayerBasicInfo,
    pub detailed_performance: DetailedPerformanceData,
    pub st_data: STRelatedData,
    pub winning_hand: WinningHandData,
}

impl Default for RaceData {
    fn default() -> Self {
        Self::new()
    }
}

impl RaceData {
    pub fn new() -> Self {
        RaceData {
            escape_last_year: 0.0,
            escape_last_half_year: 0.0,
            allow_escape_last_year: 0.0,
            allow_escape_last_half_year: 0.0,
            pierce_last_year: 0.0,
            pierce_last_half_year: 0.0,
            overtake_last_year: 0.0,
            overtake_last_half_year: 0.0,
            player_basic_info: PlayerBasicInfo::new(),
            detailed_performance: DetailedPerformanceData::new(),
            st_data: STRelatedData::new(),
            winning_hand: WinningHandData::new(),
        }
    }
}

impl fmt::Display for RaceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Escape Last Year: {:.2}%, Escape Last Half Year: {:.2}%, Allow Escape Last Year: {:.2}%, Allow Escape Last Half Year: {:.2}%, Pierce Last Year: {:.2}%, Pierce Last Half Year: {:.2}%, Overtake Last Year: {:.2}%, Overtake Last Half Year: {:.2}%",
            self.escape_last_year * 100.0,
            self.escape_last_half_year * 100.0,
            self.allow_escape_last_year * 100.0,
            self.allow_escape_last_half_year * 100.0,
            self.pierce_last_year * 100.0,
            self.pierce_last_half_year * 100.0,
            self.overtake_last_year * 100.0,
            self.overtake_last_half_year * 100.0
        )
    }
}

// ===== Odds Data =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize, PartialEq)]
pub enum BettingType {
    Trifecta,      // 3連単
    Tricast,       // 3連複
    Exacta,        // 2連単
    Quinella,      // 2連複
    QuinellaPlace, // 拡連複
    WinPlace,      // 単勝・複勝
}

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct OddsCombination {
    pub first: u8,
    pub second: u8,
    pub third: Option<u8>, // None for 2-boat combinations
    pub odds: f64,
    pub is_combined: bool, // for "合成" odds
    pub range_text: Option<String>, // 複勝オッズの場合、元の範囲文字列（例："2.4-3.5"）
}

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct OddsData {
    pub betting_type: BettingType,
    pub combinations: Vec<OddsCombination>,
}

// ===== Bulk Race Data =====

#[derive(Debug, Clone, norimaki_db::Serialize, norimaki_db::Deserialize)]
pub struct BulkRaceData {
    pub date: String,
    pub place_number: u32,
    pub race_number: u32,
    pub race_data: Option<RaceData>,
    pub win_place_odds_data: Option<OddsData>,
    pub error: Option<String>,
}

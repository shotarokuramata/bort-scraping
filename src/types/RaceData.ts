export interface PlayerBasicInfo {
  registration_number: string;
  name: string;
  class_level: string;
  period: string;
  support_group: string;
  gender: string;
}

export interface PerformanceData {
  this_period?: number;
  last_6_months?: number;
  last_3_months?: number;
  last_1_month?: number;
  local_venue?: number;
  general_races?: number;
  sg_g1?: number;
}

export interface LaneWinRateData {
  last_1_year?: number;
  last_6_months?: number;
}

export interface DetailedPerformanceData {
  first_place_rate: PerformanceData;
  lane_win_rate: LaneWinRateData;
}

export interface STData {
  this_period?: number;
  last_6_months?: number;
  last_3_months?: number;
  last_1_month?: number;
  local_venue?: number;
  general_races?: number;
  sg_g1?: number;
  first_day?: number;
  final_day?: number;
  night_races?: number;
  flying_history?: number;
}

export interface STAnalysisData {
  stability_rate?: number;
  break_out_rate?: number;
  late_start_rate?: number;
}

export interface STRelatedData {
  average_st: STData;
  st_ranking: STData;
  st_analysis: STAnalysisData;
}

export interface WinningHandData {
  escape_rate_6months?: number;
  let_escape_rate_6months?: number;
  pierced_rate_6months?: number;
  pierce_rate_6months?: number;
  overtake_rate_6months?: number;
}

export interface RaceData {
  escape_last_year: number;
  escape_last_half_year: number;
  allow_escape_last_year: number;
  allow_escape_last_half_year: number;
  pierce_last_year: number;
  pierce_last_half_year: number;
  overtake_last_year: number;
  overtake_last_half_year: number;
  player_basic_info: PlayerBasicInfo;
  detailed_performance: DetailedPerformanceData;
  st_data: STRelatedData;
  winning_hand: WinningHandData;
}
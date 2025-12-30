// バックエンドのSearchParams構造体に対応
export interface SearchParams {
  // 選手条件
  racer_number?: number;
  racer_name?: string;
  racer_class?: number; // 1=A1, 2=A2, 3=B1, 4=B2

  // 日付・会場条件
  date_from?: string; // YYYYMMDD形式
  date_to?: string;
  venue_code?: string; // "01"-"24"

  // レース条件
  race_grade?: number; // 1=SG, 2=G1, 3=G2, 4=G3, 5=一般
  race_number?: number;

  // 配当条件
  min_trifecta_payout?: number;
  max_trifecta_payout?: number;
  min_win_payout?: number;

  // 気象条件
  min_wind?: number;
  max_wind?: number;
  min_wave?: number;
  max_wave?: number;
  min_temperature?: number;
  max_temperature?: number;

  // 勝者条件
  winner_boat_number?: number;

  // 着順条件
  place_number?: number;

  // 結果数制限
  limit?: number;
}

// 正規化DBから返されるレース情報
export interface RaceRecord {
  id: number;
  race_date: string;
  venue_code: string;
  race_number: number;

  // 気象条件
  race_wind?: number;
  race_wave?: number;
  race_temperature?: number;
  race_water_temperature?: number;

  // 配当情報
  win_payout?: number;
  place_payout_max?: number;
  exacta_payout?: number;
  quinella_payout?: number;
  trifecta_payout?: number;
  trio_payout?: number;

  // 勝者情報
  winner_boat_number?: number;
  winner_racer_number?: number;

  // レース詳細
  race_grade_number?: number;
  race_title?: string;
  race_subtitle?: string;
  race_distance?: number;

  created_at: string;
  updated_at: string;
}

// 選手情報
export interface RaceParticipantRecord {
  id: number;
  race_id: number;

  // 艇・選手基本情報
  boat_number: number;
  racer_number?: number;
  racer_name?: string;

  // 級別・所属
  racer_class_number?: number;
  racer_branch_number?: number;
  racer_age?: number;
  racer_weight?: number;

  // コース・スタート情報
  course_number?: number;
  start_timing?: number;

  // レース結果
  place_number?: number;

  // 成績統計
  flying_count?: number;
  late_count?: number;
  average_start_timing?: number;
  national_top_1_percent?: number;
  national_top_2_percent?: number;
  national_top_3_percent?: number;
  local_top_1_percent?: number;
  local_top_2_percent?: number;
  local_top_3_percent?: number;

  // モーター・ボート情報
  assigned_motor_number?: number;
  assigned_motor_top_2_percent?: number;
  assigned_motor_top_3_percent?: number;
  assigned_boat_number?: number;
  assigned_boat_top_2_percent?: number;
  assigned_boat_top_3_percent?: number;

  created_at: string;
  updated_at: string;
}

// バックエンドの戻り値型 Vec<(RaceRecord, Vec<RaceParticipantRecord>)>
export type AdvancedSearchResult = [RaceRecord, RaceParticipantRecord[]];

// 検索状態管理
export interface AdvancedSearchState {
  status: "idle" | "loading" | "success" | "error";
  results: AdvancedSearchResult[];
  error: string | null;
}

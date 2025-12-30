// Open API データ管理用の型定義

export type DataType = "previews" | "results" | "programs";

export interface FetchStatus {
  previews: "idle" | "loading" | "success" | "error";
  results: "idle" | "loading" | "success" | "error";
  programs: "idle" | "loading" | "success" | "error";
}

export interface FetchError {
  previews: string | null;
  results: string | null;
  programs: string | null;
}

export interface OpenApiState {
  date: string; // YYYYMMDD format
  status: FetchStatus;
  error: FetchError;
  exportStatus: "idle" | "loading" | "success" | "error";
  exportError: string | null;
}

// 高配当検索用の型定義

export type PayoutType = "win" | "trifecta" | "exacta" | "place";

export interface PayoutEntry {
  combination?: string;
  payout?: number;
}

export interface PayoutInfo {
  win?: PayoutEntry[];
  place?: PayoutEntry[];
  exacta?: PayoutEntry[];
  quinella?: PayoutEntry[];
  quinella_place?: PayoutEntry[];
  trifecta?: PayoutEntry[];
  trio?: PayoutEntry[];
}

export interface ResultRacerInfo {
  racer_boat_number: number;
  racer_course_number?: number;
  racer_start_timing?: number;
  racer_place_number?: number;
  racer_number?: number;
  racer_name?: string;
}

export interface RaceResult {
  race_date: string;
  race_stadium_number: number;
  race_number: number;
  race_wind?: number;
  race_wind_direction_number?: number;
  race_wave?: number;
  race_weather_number?: number;
  race_temperature?: number;
  race_water_temperature?: number;
  race_technique_number?: number;
  boats: ResultRacerInfo[];
  payouts: PayoutInfo;
}

export interface PayoutStats {
  avg_trifecta?: number;
  max_trifecta?: number;
  avg_win?: number;
  max_win?: number;
}

export interface SearchState {
  status: "idle" | "loading" | "success" | "error";
  results: RaceResult[];
  error: string | null;
}

export interface StatsState {
  status: "idle" | "loading" | "success" | "error";
  stats: PayoutStats | null;
  error: string | null;
}

// データサマリー用の型定義

export interface DataSummaryRow {
  date: string; // YYYYMMDD format
  preview_count: number;
  result_count: number;
  program_count: number;
  venue_codes: string; // Comma-separated venue codes
  total_venues: number;
}

export interface SummaryState {
  status: "idle" | "loading" | "success" | "error";
  data: DataSummaryRow[];
  error: string | null;
}

// Bulk Fetch用の型定義

export interface BulkFetchSummary {
  total_days: number;
  success_count: number;
  error_count: number;
  skipped_count: number;
  errors: BulkFetchError[];
}

export interface BulkFetchError {
  date: string;
  error_message: string;
}

export interface OpenApiBulkProgressPayload {
  message: string;
  current: number;
  total: number;
  date: string;
  data_type: "previews" | "results" | "programs";
  status: "fetching" | "cached" | "saved" | "error" | "completed";
}

export interface BulkFetchState {
  status: "idle" | "loading" | "success" | "error";
  summary: BulkFetchSummary | null;
  progress: OpenApiBulkProgressPayload | null;
  error: string | null;
}

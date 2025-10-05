import { RaceData } from './RaceData';

export interface OddsCombination {
  first: number;
  second: number;
  third?: number;
  odds: number;
  is_combined: boolean;
  range_text?: string; // 複勝オッズの場合、元の範囲文字列（例："2.4-3.5"）
}

export type BettingType = "Trifecta" | "Tricast" | "Exacta" | "Quinella" | "QuinellaPlace" | "WinPlace";

export interface OddsData {
  betting_type: BettingType;
  combinations: OddsCombination[];
}

export interface BulkRaceData {
  date: string;
  place_number: number;
  race_number: number;
  race_data?: RaceData;
  win_place_odds_data?: OddsData;
  error?: string;
}
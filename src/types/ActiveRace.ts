export interface RaceVenue {
  place_id: number;
  place_name: string;
  races: number[]; // 開催レース番号のリスト
}

export interface ActiveRace {
  date: string;
  venues: RaceVenue[];
}
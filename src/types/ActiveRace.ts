export interface RaceVenue {
  place_id: number;
  place_name: string;
  races: number[]; // 開催レース番号のリスト
}

export interface ActiveRace {
  date: string;
  venues: RaceVenue[];
}

export interface VenueStatus {
  place_id: number;
  place_name: string;
  is_active: boolean;  // 開催中: true, 非開催: false
  races: number[];     // 開催中なら1-12、非開催なら空
}

export interface AllVenuesResponse {
  date: string;
  venues: VenueStatus[];
}
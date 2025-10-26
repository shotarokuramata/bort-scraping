export interface BulkProgressPayload {
  message: string;
  current: number;
  total: number;
  date: string;
  place_number: number;
  race_number: number;
  status: "cache_hit" | "scraping" | "saved" | "error" | "completed";
}

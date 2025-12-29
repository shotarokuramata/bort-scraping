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

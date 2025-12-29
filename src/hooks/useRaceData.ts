import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { RaceData } from "../types";

export function useRaceData() {
  const [raceData, setRaceData] = useState<RaceData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const fetchRaceData = async (date: string, raceNumber: string, placeNumber: string) => {
    setLoading(true);
    setError("");
    setRaceData(null);
    
    try {
      const result = await invoke<RaceData>("get_biyori_info", { date, raceNumber, placeNumber });
      console.log("✅ レースデータ取得成功:", result);
      setRaceData(result);
    } catch (err) {
      console.error("❌ レースデータ取得エラー:", err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  };

  return {
    raceData,
    loading,
    error,
    fetchRaceData,
  };
}
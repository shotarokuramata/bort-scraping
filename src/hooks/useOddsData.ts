import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { OddsData } from "../types";

export function useOddsData() {
  const [oddsData, setOddsData] = useState<OddsData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const fetchOddsData = async (date: string, raceNumber: string, placeNumber: string) => {
    setLoading(true);
    setError("");
    setOddsData(null);
    
    try {
      const result = await invoke<OddsData>("get_win_place_odds_info", { date, raceNumber, placeNumber });
      console.log("単勝・複勝オッズデータ取得成功:", result);
      setOddsData(result);
    } catch (err) {
      console.error("単勝・複勝オッズデータ取得エラー:", err);
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  return {
    oddsData,
    loading,
    error,
    fetchOddsData,
  };
}
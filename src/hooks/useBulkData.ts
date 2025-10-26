import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { BulkRaceData, BulkProgressPayload } from "../types";

export function useBulkData() {
  const [bulkData, setBulkData] = useState<BulkRaceData[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [progress, setProgress] = useState<BulkProgressPayload | null>(null);

  // プログレスイベントをリッスン
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<BulkProgressPayload>("bulk-progress", (event) => {
        setProgress(event.payload);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const fetchBulkData = async (
    startDate: string,
    endDate: string,
    selectedPlaces: number[],
    selectedRaces: number[]
  ) => {
    if (!startDate || !endDate) {
      setError("開始日と終了日を選択してください");
      return;
    }

    if (selectedPlaces.length === 0) {
      setError("少なくとも1つの競艇場を選択してください");
      return;
    }

    if (selectedRaces.length === 0) {
      setError("少なくとも1つのレース番号を選択してください");
      return;
    }

    setLoading(true);
    setError("");
    setBulkData([]);
    setProgress(null);
    
    try {
      const result = await invoke<BulkRaceData[]>("get_bulk_race_data", {
        startDate,
        endDate,
        placeNumbers: selectedPlaces,
        raceNumbers: selectedRaces,
      });
      
      setBulkData(result);
      const successCount = result.filter(item => !item.error).length;
      const errorCount = result.filter(item => item.error).length;
      console.log(`一括取得完了: 成功${successCount}件, エラー${errorCount}件`);
    } catch (err) {
      console.error("一括取得エラー:", err);
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  return {
    bulkData,
    loading,
    error,
    progress,
    fetchBulkData,
  };
}
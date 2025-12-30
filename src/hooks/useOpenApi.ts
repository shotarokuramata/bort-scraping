import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { DataType, OpenApiState, PayoutType, RaceResult, PayoutStats, SearchState, StatsState } from "../types/OpenApiData";
import { SearchParams, AdvancedSearchResult, AdvancedSearchState } from "../types/AdvancedSearch";

export const useOpenApi = () => {
  const [state, setState] = useState<OpenApiState>({
    date: formatDate(new Date()),
    status: {
      previews: "idle",
      results: "idle",
      programs: "idle",
    },
    error: {
      previews: null,
      results: null,
      programs: null,
    },
    exportStatus: "idle",
    exportError: null,
  });

  // 高配当検索用のstate
  const [searchState, setSearchState] = useState<SearchState>({
    status: "idle",
    results: [],
    error: null,
  });

  // 統計情報用のstate
  const [statsState, setStatsState] = useState<StatsState>({
    status: "idle",
    stats: null,
    error: null,
  });

  // 詳細検索用のstate
  const [advancedSearchState, setAdvancedSearchState] = useState<AdvancedSearchState>({
    status: "idle",
    results: [],
    error: null,
  });

  // 日付をYYYYMMDD形式に変換
  function formatDate(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    return `${year}${month}${day}`;
  }

  // 日付を変更
  const setDate = (date: Date) => {
    setState((prev) => ({
      ...prev,
      date: formatDate(date),
    }));
  };

  // データ取得（取得後に自動的にDBに保存）
  const fetchData = async (dataType: DataType) => {
    setState((prev) => ({
      ...prev,
      status: { ...prev.status, [dataType]: "loading" },
      error: { ...prev.error, [dataType]: null },
    }));

    try {
      // Step 1: データ取得
      let fetchCommand = "";
      if (dataType === "previews") fetchCommand = "fetch_previews_data";
      else if (dataType === "results") fetchCommand = "fetch_results_data";
      else if (dataType === "programs") fetchCommand = "fetch_programs_data";

      const jsonData = await invoke<string>(fetchCommand, { date: state.date });
      console.log(`${dataType} data fetched (${jsonData.length} bytes)`);

      // Step 2: データベースに保存
      let saveCommand = "";
      if (dataType === "previews") saveCommand = "save_previews_to_db";
      else if (dataType === "results") saveCommand = "save_results_to_db";
      else if (dataType === "programs") saveCommand = "save_programs_to_db";

      const saveCount = await invoke<number>(saveCommand, {
        date: state.date,
        jsonData,
      });
      console.log(`${dataType} data saved (${saveCount} records)`);

      setState((prev) => ({
        ...prev,
        status: { ...prev.status, [dataType]: "success" },
      }));
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error(`Failed to fetch/save ${dataType}:`, errorMessage);

      setState((prev) => ({
        ...prev,
        status: { ...prev.status, [dataType]: "error" },
        error: { ...prev.error, [dataType]: errorMessage },
      }));
    }
  };

  // CSV エクスポート (V3: 正規化スキーマ版)
  const exportToCsv = async (_dataType: DataType | "all") => {
    setState((prev) => ({
      ...prev,
      exportStatus: "loading",
      exportError: null,
    }));

    try {
      // ファイル保存ダイアログを表示
      const filePath = await save({
        defaultPath: "export.csv",
        filters: [
          {
            name: "CSV",
            extensions: ["csv"],
          },
        ],
      });

      // ユーザーがキャンセルした場合
      if (!filePath) {
        setState((prev) => ({
          ...prev,
          exportStatus: "idle",
        }));
        return;
      }

      // ファイルパスから出力ディレクトリを取得
      // 例: "/path/to/export.csv" → "/path/to"
      const outputDir = filePath.substring(0, filePath.lastIndexOf("/"));

      // V3エクスポート実行（races.csv と race_participants.csv の2ファイルを生成）
      const [raceCount, participantCount] = await invoke<[number, number]>(
        "export_open_api_to_csv_v3",
        {
          outputDir,
        }
      );

      console.log(
        `Exported ${raceCount} races and ${participantCount} participants to ${outputDir}/`
      );
      console.log(`  - ${outputDir}/races.csv`);
      console.log(`  - ${outputDir}/race_participants.csv`);

      setState((prev) => ({
        ...prev,
        exportStatus: "success",
      }));
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error("Failed to export CSV:", errorMessage);

      setState((prev) => ({
        ...prev,
        exportStatus: "error",
        exportError: errorMessage,
      }));
    }
  };

  // 高配当レース検索
  const searchHighPayoutRaces = async (
    minPayout: number,
    payoutType: PayoutType,
    limit?: number
  ) => {
    setSearchState({
      status: "loading",
      results: [],
      error: null,
    });

    try {
      const results = await invoke<RaceResult[]>("search_high_payout_races", {
        minPayout,
        payoutType,
        limit: limit || 100,
      });

      console.log(`Found ${results.length} high payout races`);

      setSearchState({
        status: "success",
        results,
        error: null,
      });

      return results;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error("Failed to search high payout races:", errorMessage);

      setSearchState({
        status: "error",
        results: [],
        error: errorMessage,
      });

      throw error;
    }
  };

  // 配当統計情報取得
  const getPayoutStatistics = async () => {
    setStatsState({
      status: "loading",
      stats: null,
      error: null,
    });

    try {
      const stats = await invoke<PayoutStats>("get_payout_statistics");

      console.log("Payout statistics:", stats);

      setStatsState({
        status: "success",
        stats,
        error: null,
      });

      return stats;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error("Failed to get payout statistics:", errorMessage);

      setStatsState({
        status: "error",
        stats: null,
        error: errorMessage,
      });

      throw error;
    }
  };

  // 詳細検索（複合条件）
  const searchAdvanced = async (params: SearchParams) => {
    setAdvancedSearchState({
      status: "loading",
      results: [],
      error: null,
    });

    try {
      const results = await invoke<AdvancedSearchResult[]>(
        "search_races_advanced",
        { params }
      );

      console.log(`Found ${results.length} races with advanced search`);

      setAdvancedSearchState({
        status: "success",
        results,
        error: null,
      });

      return results;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error("Failed to search races:", errorMessage);

      setAdvancedSearchState({
        status: "error",
        results: [],
        error: errorMessage,
      });

      throw error;
    }
  };

  return {
    date: state.date,
    status: state.status,
    error: state.error,
    exportStatus: state.exportStatus,
    exportError: state.exportError,
    searchState,
    statsState,
    advancedSearchState,
    setDate,
    fetchData,
    exportToCsv,
    searchHighPayoutRaces,
    getPayoutStatistics,
    searchAdvanced,
  };
};

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DataType, OpenApiState } from "../types/OpenApiData";

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

  // サービス初期化（バックエンドでデフォルトパスを使用）
  useEffect(() => {
    const initService = async () => {
      try {
        await invoke("init_open_api_service");
        console.log("Open API service initialized");
      } catch (error) {
        console.error("Failed to initialize Open API service:", error);
      }
    };
    initService();
  }, []);

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

  // CSV エクスポート
  const exportToCsv = async (dataType: DataType | "all", outputPath: string) => {
    setState((prev) => ({
      ...prev,
      exportStatus: "loading",
      exportError: null,
    }));

    try {
      const dataTypeParam = dataType === "all" ? null : dataType;
      const count = await invoke<number>("export_open_api_to_csv", {
        outputPath,
        dataType: dataTypeParam,
      });

      console.log(`Exported ${count} records to CSV`);

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

  return {
    date: state.date,
    status: state.status,
    error: state.error,
    exportStatus: state.exportStatus,
    exportError: state.exportError,
    setDate,
    fetchData,
    exportToCsv,
  };
};

import { useState } from "react";
import { useOpenApi } from "../hooks/useOpenApi";
import { DataType } from "../types/OpenApiData";

const OpenApiTool = () => {
  const {
    date,
    status,
    error,
    exportStatus,
    exportError,
    setDate,
    fetchData,
    exportToCsv,
  } = useOpenApi();

  const [selectedExportType, setSelectedExportType] = useState<DataType | "all">("all");

  // 日付変更ハンドラー
  const handleDateChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const inputDate = e.target.value; // YYYY-MM-DD
    const dateObj = new Date(inputDate);
    setDate(dateObj);
  };

  // 現在の日付をYYYY-MM-DD形式に変換（input[type="date"]用）
  const getFormattedDateForInput = () => {
    const year = date.slice(0, 4);
    const month = date.slice(4, 6);
    const day = date.slice(6, 8);
    return `${year}-${month}-${day}`;
  };

  // ステータス表示用
  const getStatusText = (type: DataType) => {
    const statusText = {
      idle: "未取得",
      loading: "取得中...",
      success: "✅ 取得完了",
      error: `❌ エラー: ${error[type]}`,
    };
    return statusText[status[type]];
  };

  // エクスポートハンドラー
  const handleExport = () => {
    exportToCsv(selectedExportType);
  };

  return (
    <div style={{ padding: "20px", maxWidth: "800px", margin: "0 auto" }}>
      <h1>Open API データ管理</h1>

      {/* 日付選択 */}
      <div style={{ marginBottom: "30px" }}>
        <h2>日付選択</h2>
        <input
          type="date"
          value={getFormattedDateForInput()}
          onChange={handleDateChange}
          style={{
            padding: "8px",
            fontSize: "16px",
            border: "1px solid #ccc",
            borderRadius: "4px",
          }}
        />
        <span style={{ marginLeft: "10px", color: "#666" }}>
          選択中: {date}
        </span>
      </div>

      {/* データ取得 */}
      <div style={{ marginBottom: "30px" }}>
        <h2>データ取得</h2>
        <div style={{ display: "flex", gap: "10px", marginBottom: "15px" }}>
          <button
            onClick={() => fetchData("previews")}
            disabled={status.previews === "loading"}
            style={{
              padding: "10px 20px",
              fontSize: "16px",
              backgroundColor: "#4CAF50",
              color: "white",
              border: "none",
              borderRadius: "4px",
              cursor: status.previews === "loading" ? "not-allowed" : "pointer",
              opacity: status.previews === "loading" ? 0.6 : 1,
            }}
          >
            Previews 取得
          </button>
          <button
            onClick={() => fetchData("results")}
            disabled={status.results === "loading"}
            style={{
              padding: "10px 20px",
              fontSize: "16px",
              backgroundColor: "#2196F3",
              color: "white",
              border: "none",
              borderRadius: "4px",
              cursor: status.results === "loading" ? "not-allowed" : "pointer",
              opacity: status.results === "loading" ? 0.6 : 1,
            }}
          >
            Results 取得
          </button>
          <button
            onClick={() => fetchData("programs")}
            disabled={status.programs === "loading"}
            style={{
              padding: "10px 20px",
              fontSize: "16px",
              backgroundColor: "#FF9800",
              color: "white",
              border: "none",
              borderRadius: "4px",
              cursor: status.programs === "loading" ? "not-allowed" : "pointer",
              opacity: status.programs === "loading" ? 0.6 : 1,
            }}
          >
            Programs 取得
          </button>
        </div>

        {/* ステータス表示 */}
        <div style={{ padding: "15px", backgroundColor: "#f5f5f5", borderRadius: "4px" }}>
          <div><strong>Previews:</strong> {getStatusText("previews")}</div>
          <div><strong>Results:</strong> {getStatusText("results")}</div>
          <div><strong>Programs:</strong> {getStatusText("programs")}</div>
        </div>
      </div>

      <hr style={{ margin: "30px 0", border: "none", borderTop: "1px solid #ddd" }} />

      {/* CSV エクスポート */}
      <div>
        <h2>CSV エクスポート</h2>
        <div style={{ marginBottom: "15px" }}>
          <label style={{ display: "block", marginBottom: "5px" }}>
            データ種類:
          </label>
          <select
            value={selectedExportType}
            onChange={(e) => setSelectedExportType(e.target.value as DataType | "all")}
            style={{
              padding: "8px",
              fontSize: "16px",
              border: "1px solid #ccc",
              borderRadius: "4px",
              minWidth: "200px",
            }}
          >
            <option value="all">すべて</option>
            <option value="previews">Previews のみ</option>
            <option value="results">Results のみ</option>
            <option value="programs">Programs のみ</option>
          </select>
        </div>

        <button
          onClick={handleExport}
          disabled={exportStatus === "loading"}
          style={{
            padding: "10px 30px",
            fontSize: "16px",
            backgroundColor: "#9C27B0",
            color: "white",
            border: "none",
            borderRadius: "4px",
            cursor: exportStatus === "loading" ? "not-allowed" : "pointer",
            opacity: exportStatus === "loading" ? 0.6 : 1,
          }}
        >
          {exportStatus === "loading" ? "出力中..." : "保存先を選択して CSV 出力"}
        </button>

        {/* エクスポートステータス */}
        {exportStatus === "success" && (
          <div style={{ marginTop: "10px", color: "#4CAF50" }}>
            ✅ CSV出力が完了しました（選択した場所に保存されました）
          </div>
        )}
        {exportStatus === "error" && (
          <div style={{ marginTop: "10px", color: "#f44336" }}>
            ❌ エラー: {exportError}
          </div>
        )}
      </div>

      {/* ホームに戻るリンク */}
      <div style={{ marginTop: "40px" }}>
        <a href="/" style={{ color: "#2196F3", textDecoration: "none" }}>
          ← ホームに戻る
        </a>
      </div>
    </div>
  );
};

export default OpenApiTool;

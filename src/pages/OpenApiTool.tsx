import { useEffect, useState } from "react";
import { useOpenApi } from "../hooks/useOpenApi";
import { DataType } from "../types/OpenApiData";
import { DataSummaryDisplay } from "../components/parts/DataSummaryDisplay";

const OpenApiTool = () => {
  const {
    date,
    status,
    error,
    exportStatus,
    exportError,
    summaryState,
    bulkFetchState,
    setDate,
    fetchData,
    exportToCsv,
    fetchDataSummary,
    fetchDataBulk,
  } = useOpenApi();

  // Bulk Fetch用のstate
  const [bulkStartDate, setBulkStartDate] = useState("");
  const [bulkEndDate, setBulkEndDate] = useState("");
  const [bulkDataType, setBulkDataType] = useState<DataType>("previews");

  // コンポーネントマウント時にサマリーを取得
  useEffect(() => {
    fetchDataSummary().catch((err) => {
      console.error("Failed to fetch initial summary:", err);
    });
  }, []);

  // 日付変更ハンドラー
  const handleDateChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const inputDate = e.target.value; // YYYY-MM-DD
    const dateObj = new Date(inputDate);
    setDate(dateObj);
  };

  // データ取得ハンドラー（取得後にサマリーを更新）
  const handleFetchData = async (dataType: DataType) => {
    try {
      await fetchData(dataType);
      // データ取得成功後、サマリーを自動更新
      await fetchDataSummary();
    } catch (err) {
      console.error(`Failed to fetch ${dataType}:`, err);
    }
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

  // エクスポートハンドラー（V3では常に全データをエクスポート）
  const handleExport = () => {
    exportToCsv("all");
  };

  // Bulk Fetch ハンドラー
  const handleBulkFetch = async () => {
    try {
      const summary = await fetchDataBulk(
        bulkDataType,
        bulkStartDate.replace(/-/g, ""),  // YYYY-MM-DD → YYYYMMDD
        bulkEndDate.replace(/-/g, "")
      );
      console.log("Bulk fetch completed:", summary);
      // 取得後にサマリーを更新
      await fetchDataSummary();
    } catch (err) {
      console.error("Bulk fetch failed:", err);
    }
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
            onClick={() => handleFetchData("previews")}
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
            onClick={() => handleFetchData("results")}
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
            onClick={() => handleFetchData("programs")}
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

      {/* 期間一括取得 */}
      <div style={{ marginBottom: "30px" }}>
        <h2>期間一括取得</h2>
        <p style={{ color: "#666", fontSize: "14px", marginBottom: "15px" }}>
          指定した期間のデータを自動的に取得します。既にデータベースに存在する日付はスキップされます。
        </p>

        <div style={{ display: "flex", gap: "15px", marginBottom: "15px", alignItems: "flex-end", flexWrap: "wrap" }}>
          <div>
            <label style={{ display: "block", marginBottom: "5px", fontSize: "14px" }}>データ種別:</label>
            <select
              value={bulkDataType}
              onChange={(e) => setBulkDataType(e.target.value as DataType)}
              style={{ padding: "8px", fontSize: "16px", borderRadius: "4px", border: "1px solid #ccc" }}
            >
              <option value="previews">Previews</option>
              <option value="results">Results</option>
              <option value="programs">Programs</option>
            </select>
          </div>

          <div>
            <label style={{ display: "block", marginBottom: "5px", fontSize: "14px" }}>開始日:</label>
            <input
              type="date"
              value={bulkStartDate}
              onChange={(e) => setBulkStartDate(e.target.value)}
              style={{ padding: "8px", fontSize: "16px", borderRadius: "4px", border: "1px solid #ccc" }}
            />
          </div>

          <div>
            <label style={{ display: "block", marginBottom: "5px", fontSize: "14px" }}>終了日:</label>
            <input
              type="date"
              value={bulkEndDate}
              onChange={(e) => setBulkEndDate(e.target.value)}
              style={{ padding: "8px", fontSize: "16px", borderRadius: "4px", border: "1px solid #ccc" }}
            />
          </div>

          <button
            onClick={handleBulkFetch}
            disabled={bulkFetchState.status === "loading" || !bulkStartDate || !bulkEndDate}
            style={{
              padding: "10px 20px",
              fontSize: "16px",
              backgroundColor: "#9C27B0",
              color: "white",
              border: "none",
              borderRadius: "4px",
              cursor: (bulkFetchState.status === "loading" || !bulkStartDate || !bulkEndDate) ? "not-allowed" : "pointer",
              opacity: (bulkFetchState.status === "loading" || !bulkStartDate || !bulkEndDate) ? 0.6 : 1,
            }}
          >
            {bulkFetchState.status === "loading" ? "取得中..." : "一括取得開始"}
          </button>
        </div>

        {/* プログレス表示 */}
        {bulkFetchState.progress && (
          <div style={{ marginTop: "15px", padding: "15px", backgroundColor: "#f5f5f5", borderRadius: "4px" }}>
            <div style={{ marginBottom: "10px", fontSize: "14px" }}>{bulkFetchState.progress.message}</div>
            <div style={{ marginBottom: "5px", fontSize: "14px" }}>
              進捗: {bulkFetchState.progress.current} / {bulkFetchState.progress.total} 日
            </div>
            <progress
              value={bulkFetchState.progress.current}
              max={bulkFetchState.progress.total}
              style={{ width: "100%", height: "20px" }}
            />
          </div>
        )}

        {/* サマリー表示 */}
        {bulkFetchState.summary && (
          <div style={{ marginTop: "15px", padding: "15px", backgroundColor: "#e8f5e9", borderRadius: "4px" }}>
            <h3 style={{ marginTop: 0 }}>取得完了サマリー</h3>
            <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "10px", fontSize: "14px" }}>
              <div>対象日数: {bulkFetchState.summary.total_days} 日</div>
              <div>成功: {bulkFetchState.summary.success_count} 日</div>
              <div>スキップ: {bulkFetchState.summary.skipped_count} 日 (既存)</div>
              <div>エラー: {bulkFetchState.summary.error_count} 日</div>
            </div>

            {bulkFetchState.summary.errors.length > 0 && (
              <details style={{ marginTop: "10px" }}>
                <summary style={{ cursor: "pointer", fontWeight: "bold", fontSize: "14px" }}>
                  エラー詳細 ({bulkFetchState.summary.errors.length}件)
                </summary>
                <ul style={{ marginTop: "10px", paddingLeft: "20px", fontSize: "13px" }}>
                  {bulkFetchState.summary.errors.map((err, i) => (
                    <li key={i} style={{ marginBottom: "5px" }}>
                      <strong>{err.date}</strong>: {err.error_message}
                    </li>
                  ))}
                </ul>
              </details>
            )}
          </div>
        )}

        {/* エラー表示 */}
        {bulkFetchState.error && (
          <div style={{ marginTop: "15px", padding: "15px", backgroundColor: "#ffebee", borderRadius: "4px", color: "#c62828", fontSize: "14px" }}>
            ❌ エラー: {bulkFetchState.error}
          </div>
        )}
      </div>

      <hr style={{ margin: "30px 0", border: "none", borderTop: "1px solid #ddd" }} />

      {/* データサマリー表示 */}
      <DataSummaryDisplay
        data={summaryState.data}
        isLoading={summaryState.status === "loading"}
        error={summaryState.error}
        onRefresh={() => fetchDataSummary()}
      />

      <hr style={{ margin: "30px 0", border: "none", borderTop: "1px solid #ddd" }} />

      {/* CSV エクスポート */}
      <div>
        <h2>CSV エクスポート (V3 正規化スキーマ版)</h2>
        <div style={{ marginBottom: "15px", padding: "10px", backgroundColor: "#f5f5f5", borderRadius: "4px" }}>
          <p style={{ margin: "0 0 8px 0", fontSize: "14px" }}>
            📊 <strong>エクスポート形式：</strong>
          </p>
          <ul style={{ margin: "0", paddingLeft: "20px", fontSize: "14px" }}>
            <li><strong>races.csv</strong> - レース情報（配当、天候、勝者など）</li>
            <li><strong>race_participants.csv</strong> - 参加者情報（選手、成績、モーター/ボート統計など）</li>
          </ul>
          <p style={{ margin: "8px 0 0 0", fontSize: "13px", color: "#666" }}>
            ※ JSONカラムは除外され、すべてのデータが個別のカラムに展開されます
          </p>
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
          <div style={{ marginTop: "10px", padding: "10px", backgroundColor: "#e8f5e9", borderRadius: "4px" }}>
            <div style={{ color: "#4CAF50", fontWeight: "bold" }}>
              ✅ CSV出力が完了しました
            </div>
            <div style={{ fontSize: "14px", marginTop: "5px", color: "#555" }}>
              以下の2つのファイルが生成されました：
              <ul style={{ margin: "5px 0 0 0", paddingLeft: "20px" }}>
                <li>races.csv</li>
                <li>race_participants.csv</li>
              </ul>
            </div>
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

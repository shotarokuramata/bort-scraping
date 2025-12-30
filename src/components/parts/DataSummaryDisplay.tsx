import { DataSummaryRow } from "../../types/OpenApiData";

interface DataSummaryDisplayProps {
  data: DataSummaryRow[];
  isLoading: boolean;
  error: string | null;
  onRefresh: () => void;
}

export const DataSummaryDisplay = ({
  data,
  isLoading,
  error,
  onRefresh,
}: DataSummaryDisplayProps) => {
  // 日付をYYYY-MM-DD形式にフォーマット
  const formatDate = (dateStr: string): string => {
    if (dateStr.length !== 8) return dateStr;
    const year = dateStr.slice(0, 4);
    const month = dateStr.slice(4, 6);
    const day = dateStr.slice(6, 8);
    return `${year}-${month}-${day}`;
  };

  // 総統計を計算
  const totalDates = data.length;
  const totalRecords = data.reduce(
    (sum, row) => sum + row.preview_count + row.result_count + row.program_count,
    0
  );

  return (
    <div style={{ marginBottom: "30px" }}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          marginBottom: "15px",
        }}
      >
        <h2>データ取得状況サマリー</h2>
        <button
          onClick={onRefresh}
          disabled={isLoading}
          style={{
            padding: "8px 16px",
            backgroundColor: "#9C27B0",
            color: "white",
            border: "none",
            borderRadius: "4px",
            cursor: isLoading ? "not-allowed" : "pointer",
            fontSize: "14px",
            opacity: isLoading ? 0.6 : 1,
          }}
        >
          {isLoading ? "更新中..." : "🔄 更新"}
        </button>
      </div>

      {/* エラー表示 */}
      {error && (
        <div
          style={{
            padding: "15px",
            backgroundColor: "#ffebee",
            border: "1px solid #ef5350",
            borderRadius: "4px",
            marginBottom: "15px",
            color: "#c62828",
          }}
        >
          ❌ エラー: {error}
        </div>
      )}

      {/* ローディング表示 */}
      {isLoading && (
        <div
          style={{
            padding: "20px",
            textAlign: "center",
            backgroundColor: "#f5f5f5",
            borderRadius: "4px",
            marginBottom: "15px",
          }}
        >
          データを読み込み中...
        </div>
      )}

      {/* データ表示 */}
      {!isLoading && !error && (
        <>
          {/* 総統計 */}
          {data.length > 0 && (
            <div
              style={{
                padding: "15px",
                backgroundColor: "#e3f2fd",
                borderRadius: "4px",
                marginBottom: "15px",
                display: "flex",
                gap: "30px",
                fontSize: "14px",
              }}
            >
              <div>
                <strong>全期間:</strong> {totalDates}日
              </div>
              <div>
                <strong>総レコード:</strong> {totalRecords.toLocaleString()}件
              </div>
            </div>
          )}

          {/* 空状態 */}
          {data.length === 0 && (
            <div
              style={{
                padding: "30px",
                textAlign: "center",
                backgroundColor: "#f5f5f5",
                borderRadius: "4px",
                color: "#666",
              }}
            >
              <p style={{ marginBottom: "10px" }}>
                📊 取得済みデータがありません
              </p>
              <p style={{ fontSize: "14px" }}>
                日付を選択してデータを取得してください
              </p>
            </div>
          )}

          {/* 日付別リスト */}
          {data.length > 0 && (
            <div
              style={{
                border: "1px solid #ddd",
                borderRadius: "4px",
                backgroundColor: "white",
              }}
            >
              {data.map((row, index) => (
                <div
                  key={row.date}
                  style={{
                    padding: "15px",
                    borderBottom:
                      index < data.length - 1 ? "1px solid #eee" : "none",
                  }}
                >
                  {/* 日付ヘッダー */}
                  <div
                    style={{
                      display: "flex",
                      alignItems: "center",
                      marginBottom: "10px",
                      fontSize: "16px",
                      fontWeight: "bold",
                    }}
                  >
                    📅 {formatDate(row.date)}
                    <span
                      style={{
                        marginLeft: "10px",
                        fontSize: "14px",
                        fontWeight: "normal",
                        color: "#666",
                      }}
                    >
                      (
                      {row.preview_count +
                        row.result_count +
                        row.program_count}
                      レース)
                    </span>
                  </div>

                  {/* データ種別カウント */}
                  <div
                    style={{
                      display: "flex",
                      gap: "10px",
                      marginBottom: "10px",
                      flexWrap: "wrap",
                    }}
                  >
                    <span
                      style={{
                        padding: "4px 12px",
                        backgroundColor: "#4CAF50",
                        color: "white",
                        borderRadius: "12px",
                        fontSize: "13px",
                      }}
                    >
                      Previews: {row.preview_count}
                    </span>
                    <span
                      style={{
                        padding: "4px 12px",
                        backgroundColor: "#2196F3",
                        color: "white",
                        borderRadius: "12px",
                        fontSize: "13px",
                      }}
                    >
                      Results: {row.result_count}
                    </span>
                    <span
                      style={{
                        padding: "4px 12px",
                        backgroundColor: "#FF9800",
                        color: "white",
                        borderRadius: "12px",
                        fontSize: "13px",
                      }}
                    >
                      Programs: {row.program_count}
                    </span>
                  </div>

                  {/* 会場コード */}
                  {row.venue_codes && (
                    <div
                      style={{
                        fontSize: "13px",
                        color: "#666",
                      }}
                    >
                      会場: {row.venue_codes}
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
};

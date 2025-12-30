import { useEffect } from "react";
import { useOpenApi } from "../hooks/useOpenApi";
import { SearchParams } from "../types/AdvancedSearch";
import { AdvancedSearchForm } from "../components/forms/AdvancedSearchForm";
import { AdvancedRaceResultCard } from "../components/parts/AdvancedRaceResultCard";
import { PayoutStatsDisplay } from "../components/parts/PayoutStatsDisplay";

const HighPayoutSearch = () => {
  const {
    statsState,
    advancedSearchState,
    getPayoutStatistics,
    searchAdvanced,
  } = useOpenApi();

  // 初期ロード時に統計情報を取得
  useEffect(() => {
    getPayoutStatistics();
  }, []);

  const handleAdvancedSearch = async (params: SearchParams) => {
    try {
      await searchAdvanced(params);
    } catch (error) {
      console.error("Advanced search failed:", error);
    }
  };

  return (
    <div style={{ padding: "20px", maxWidth: "1400px", margin: "0 auto" }}>
      <h1>レース詳細検索</h1>

      {/* 統計情報表示 */}
      <div style={{ marginBottom: "30px" }}>
        <PayoutStatsDisplay
          stats={statsState.stats}
          isLoading={statsState.status === "loading"}
          error={statsState.error}
          onRefresh={getPayoutStatistics}
        />
      </div>

      <hr style={{ margin: "30px 0", border: "none", borderTop: "1px solid #ddd" }} />

      {/* 検索フォーム */}
      <div style={{ marginBottom: "30px" }}>
        <h2>検索条件</h2>
        <AdvancedSearchForm
          onSearch={handleAdvancedSearch}
          isLoading={advancedSearchState.status === "loading"}
        />
      </div>

      {/* 検索結果 */}
      <div>
        <h2>
          検索結果
          {advancedSearchState.results.length > 0 && (
            <span style={{ marginLeft: "10px", fontSize: "16px", color: "#666" }}>
              ({advancedSearchState.results.length}件)
            </span>
          )}
        </h2>

        {advancedSearchState.status === "loading" && (
          <div style={{ padding: "40px", textAlign: "center", color: "#666" }}>
            検索中...
          </div>
        )}

        {advancedSearchState.status === "error" && (
          <div
            style={{
              padding: "20px",
              backgroundColor: "#ffebee",
              color: "#c62828",
              borderRadius: "4px",
            }}
          >
            エラー: {advancedSearchState.error}
          </div>
        )}

        {advancedSearchState.status === "success" && advancedSearchState.results.length === 0 && (
          <div
            style={{
              padding: "40px",
              textAlign: "center",
              color: "#666",
              backgroundColor: "#f5f5f5",
              borderRadius: "4px",
            }}
          >
            検索条件に一致するレースが見つかりませんでした
          </div>
        )}

        {advancedSearchState.status === "success" && advancedSearchState.results.length > 0 && (
          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(auto-fill, minmax(500px, 1fr))",
              gap: "20px",
            }}
          >
            {advancedSearchState.results.map((result, index) => (
              <AdvancedRaceResultCard
                key={result[0].id}
                result={result}
                index={index}
              />
            ))}
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

export default HighPayoutSearch;

import { useEffect } from "react";
import { useOpenApi } from "../hooks/useOpenApi";
import { PayoutType } from "../types/OpenApiData";
import { HighPayoutSearchForm } from "../components/forms/HighPayoutSearchForm";
import { RaceResultCard } from "../components/parts/RaceResultCard";
import { PayoutStatsDisplay } from "../components/parts/PayoutStatsDisplay";

const HighPayoutSearch = () => {
  const {
    searchState,
    statsState,
    searchHighPayoutRaces,
    getPayoutStatistics,
  } = useOpenApi();

  // 初期ロード時に統計情報を取得
  useEffect(() => {
    getPayoutStatistics();
  }, []);

  const handleSearch = async (
    minPayout: number,
    payoutType: PayoutType,
    limit: number
  ) => {
    try {
      await searchHighPayoutRaces(minPayout, payoutType, limit);
    } catch (error) {
      console.error("Search failed:", error);
    }
  };

  return (
    <div style={{ padding: "20px", maxWidth: "1200px", margin: "0 auto" }}>
      <h1>高配当レース検索</h1>

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
        <HighPayoutSearchForm
          onSearch={handleSearch}
          isLoading={searchState.status === "loading"}
        />
      </div>

      {/* 検索結果 */}
      <div>
        <h2>
          検索結果
          {searchState.results.length > 0 && (
            <span style={{ marginLeft: "10px", fontSize: "16px", color: "#666" }}>
              ({searchState.results.length}件)
            </span>
          )}
        </h2>

        {searchState.status === "loading" && (
          <div style={{ padding: "40px", textAlign: "center", color: "#666" }}>
            検索中...
          </div>
        )}

        {searchState.status === "error" && (
          <div
            style={{
              padding: "20px",
              backgroundColor: "#ffebee",
              color: "#c62828",
              borderRadius: "4px",
            }}
          >
            エラー: {searchState.error}
          </div>
        )}

        {searchState.status === "success" && searchState.results.length === 0 && (
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

        {searchState.status === "success" && searchState.results.length > 0 && (
          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(auto-fill, minmax(400px, 1fr))",
              gap: "20px",
            }}
          >
            {searchState.results.map((result, index) => (
              <RaceResultCard
                key={`${result.race_date}-${result.race_stadium_number}-${result.race_number}`}
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

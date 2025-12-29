import React from "react";
import { PayoutStats } from "../../types/OpenApiData";

interface PayoutStatsDisplayProps {
  stats: PayoutStats | null;
  isLoading: boolean;
  error: string | null;
  onRefresh: () => void;
}

export const PayoutStatsDisplay: React.FC<PayoutStatsDisplayProps> = ({
  stats,
  isLoading,
  error,
  onRefresh,
}) => {
  if (isLoading) {
    return (
      <div className="stats-display loading">
        <p>統計情報を読み込み中...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="stats-display error">
        <p>エラー: {error}</p>
        <button onClick={onRefresh}>再読み込み</button>
      </div>
    );
  }

  if (!stats) {
    return (
      <div className="stats-display empty">
        <p>統計情報を読み込んでください</p>
        <button onClick={onRefresh}>読み込み</button>
      </div>
    );
  }

  return (
    <div className="stats-display">
      <div className="stats-header">
        <h3>配当統計情報</h3>
        <button onClick={onRefresh} className="refresh-button">
          更新
        </button>
      </div>

      <div className="stats-grid">
        <div className="stat-card">
          <h4>3連単</h4>
          <div className="stat-row">
            <span className="stat-label">平均配当:</span>
            <span className="stat-value">
              ¥{stats.avg_trifecta?.toLocaleString("ja-JP", { maximumFractionDigits: 0 }) || "-"}
            </span>
          </div>
          <div className="stat-row">
            <span className="stat-label">最高配当:</span>
            <span className="stat-value highlight">
              ¥{stats.max_trifecta?.toLocaleString("ja-JP") || "-"}
            </span>
          </div>
        </div>

        <div className="stat-card">
          <h4>単勝</h4>
          <div className="stat-row">
            <span className="stat-label">平均配当:</span>
            <span className="stat-value">
              ¥{stats.avg_win?.toLocaleString("ja-JP", { maximumFractionDigits: 0 }) || "-"}
            </span>
          </div>
          <div className="stat-row">
            <span className="stat-label">最高配当:</span>
            <span className="stat-value highlight">
              ¥{stats.max_win?.toLocaleString("ja-JP") || "-"}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

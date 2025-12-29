import { OddsData } from "../../types";
import { OddsItem } from "../parts";

interface OddsDataContainerProps {
  oddsData: OddsData | null;
}

export function OddsDataContainer({ oddsData }: OddsDataContainerProps) {
  if (!oddsData) {
    console.log("⚠️ OddsDataContainer: データなし");
    return null;
  }

  console.log("🎨 OddsDataContainer レンダリング開始:", oddsData);

  return (
    <div className="race-data">
      <h2>単勝・複勝オッズデータ</h2>

      <div className="odds-summary">
        <p>総オッズ数: {oddsData.combinations.length}個</p>
      </div>

      <div className="odds-grid">
        {/* 単勝オッズ */}
        <div className="odds-card">
          <h3>単勝オッズ</h3>
          <div className="odds-list">
            {oddsData.combinations
              .filter(combo => combo.second === 0) // 単勝オッズ
              .sort((a, b) => a.first - b.first) // 艇番順でソート
              .map((combo, index) => (
                <OddsItem
                  key={index}
                  boatNumber={combo.first}
                  odds={combo.odds}
                />
              ))}
          </div>
        </div>

        {/* 複勝オッズ */}
        <div className="odds-card">
          <h3>複勝オッズ</h3>
          <div className="odds-list">
            {oddsData.combinations
              .filter(combo => combo.second === 1) // 複勝オッズ
              .sort((a, b) => a.first - b.first) // 艇番順でソート
              .map((combo, index) => (
                <OddsItem
                  key={index}
                  boatNumber={combo.first}
                  odds={combo.odds}
                  rangeText={combo.range_text}
                />
              ))}
          </div>
        </div>
      </div>
    </div>
  );
}
import { BulkRaceData } from "../../types";
import { kyoteiPlaces } from "../../information";

interface BulkResultsContainerProps {
  bulkData: BulkRaceData[];
}

export function BulkResultsContainer({ bulkData }: BulkResultsContainerProps) {
  if (bulkData.length === 0) return null;

  return (
    <div className="bulk-results">
      <h2>一括取得結果</h2>
      <div className="bulk-summary">
        <p>
          総数: {bulkData.length}件, 
          成功: {bulkData.filter(item => !item.error).length}件, 
          エラー: {bulkData.filter(item => item.error).length}件
        </p>
      </div>
      <div className="bulk-data-list">
        {bulkData.map((item, index) => (
          <div key={index} className={`bulk-data-item ${item.error ? 'error' : 'success'}`}>
            <div className="bulk-item-header">
              <span className="date">{item.date}</span>
              <span className="place">場所: {kyoteiPlaces[item.place_number as keyof typeof kyoteiPlaces] || item.place_number}</span>
              <span className="race">レース: {item.race_number}R</span>
            </div>
            {item.error ? (
              <div className="bulk-item-error">
                エラー: {item.error}
              </div>
            ) : (
              <div className="bulk-item-success">
                {/* 基本レースデータ */}
                {item.race_data && (
                  <div className="bulk-race-details">
                    <div className="bulk-race-summary">
                      <strong>選手情報:</strong> {item.race_data.player_basic_info.name} ({item.race_data.player_basic_info.class_level}) - {item.race_data.player_basic_info.registration_number}
                    </div>
                    <div className="bulk-race-stats">
                      <div className="stat-row">
                        <span>逃げ率(半年): {(item.race_data.escape_last_half_year * 100).toFixed(1)}%</span>
                        <span>逃げ率(1年): {(item.race_data.escape_last_year * 100).toFixed(1)}%</span>
                      </div>
                      <div className="stat-row">
                        <span>逃がし率(半年): {(item.race_data.allow_escape_last_half_year * 100).toFixed(1)}%</span>
                        <span>逃がし率(1年): {(item.race_data.allow_escape_last_year * 100).toFixed(1)}%</span>
                      </div>
                      <div className="stat-row">
                        <span>差され率(半年): {(item.race_data.pierce_last_half_year * 100).toFixed(1)}%</span>
                        <span>捲られ率(半年): {(item.race_data.overtake_last_half_year * 100).toFixed(1)}%</span>
                      </div>
                    </div>
                  </div>
                )}
                
                {/* 単勝・複勝データ */}
                {item.win_place_odds_data && (
                  <div className="bulk-win-place-details">
                    <div className="bulk-win-place-summary">
                      <strong>単勝・複勝オッズ:</strong> {item.win_place_odds_data.combinations.length}パターン
                    </div>
                    <div className="bulk-win-place-highlights">
                      <div className="win-place-highlight">
                        <span>単勝最低: </span>
                        {(() => {
                          const winOdds = item.win_place_odds_data.combinations.filter(c => c.second === 0);
                          if (winOdds.length === 0) return 'なし';
                          const minWin = winOdds.reduce((min, c) => c.odds < min.odds ? c : min);
                          return `${minWin.first}号艇 (${minWin.odds.toFixed(1)})`;
                        })()}
                      </div>
                      <div className="win-place-highlight">
                        <span>複勝最低: </span>
                        {(() => {
                          const placeOdds = item.win_place_odds_data.combinations.filter(c => c.second === 1);
                          if (placeOdds.length === 0) return 'なし';
                          const minPlace = placeOdds.reduce((min, c) => c.odds < min.odds ? c : min);
                          return `${minPlace.first}号艇 (${minPlace.range_text || minPlace.odds.toFixed(1)})`;
                        })()}
                      </div>
                    </div>
                  </div>
                )}
                
                <div className="bulk-data-status">
                  {item.race_data ? '✓ レースデータ' : '✗ レースデータなし'}
                  {item.win_place_odds_data ? ', ✓ 単複' : ', ✗ 単複なし'}
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
import React, { useState } from "react";
import { AdvancedSearchResult } from "../../types/AdvancedSearch";
import { kyoteiPlaces } from "../../information";
import { formatRaceDate, getRacerClassName, formatPayout } from "../../utils/formatters";

interface AdvancedRaceResultCardProps {
  result: AdvancedSearchResult;
  index: number;
}

export const AdvancedRaceResultCard: React.FC<AdvancedRaceResultCardProps> = ({
  result,
  index,
}) => {
  const [race, participants] = result;
  const [expanded, setExpanded] = useState(false);

  // 上位3名の選手を取得（着順順にソート）
  const topThree = participants
    .filter((p) => p.place_number && p.place_number <= 3)
    .sort((a, b) => (a.place_number || 0) - (b.place_number || 0));

  // 全選手を艇番順にソート
  const allParticipants = [...participants].sort(
    (a, b) => a.boat_number - b.boat_number
  );

  // 会場名を取得
  const venueNumber = parseInt(race.venue_code, 10);
  const venueName = kyoteiPlaces[venueNumber] || `不明(${race.venue_code})`;

  return (
    <div className="race-result-card">
      {/* ヘッダー */}
      <div className="card-header">
        <span className="rank">#{index + 1}</span>
        <span className="date">{formatRaceDate(race.race_date)}</span>
        <span className="venue">{venueName}</span>
        <span className="race-number">{race.race_number}R</span>
      </div>

      <div className="card-body">
        {/* 配当情報 */}
        <div className="payouts">
          <h4>配当</h4>
          <div className="payout-row">
            <span className="payout-label">3連単:</span>
            <span className="payout-value">{formatPayout(race.trifecta_payout)}</span>
          </div>
          <div className="payout-row">
            <span className="payout-label">2連単:</span>
            <span className="payout-value">{formatPayout(race.exacta_payout)}</span>
          </div>
          <div className="payout-row">
            <span className="payout-label">単勝:</span>
            <span className="payout-value">{formatPayout(race.win_payout)}</span>
          </div>
          <div className="payout-row">
            <span className="payout-label">複勝:</span>
            <span className="payout-value">{formatPayout(race.place_payout_max)}</span>
          </div>
        </div>

        {/* 着順（上位3名）*/}
        {topThree.length > 0 && (
          <div className="winners">
            <h4>着順</h4>
            {topThree.map((p) => (
              <div key={p.boat_number} className="winner-row">
                <span className="place">{p.place_number}着</span>
                <span className="boat-number">{p.boat_number}号艇</span>
                <span className="racer-name">{p.racer_name || "不明"}</span>
                <span className="racer-class">
                  ({getRacerClassName(p.racer_class_number)})
                </span>
              </div>
            ))}
          </div>
        )}

        {/* レース条件（気象情報） */}
        {(race.race_wind !== undefined ||
          race.race_wave !== undefined ||
          race.race_temperature !== undefined) && (
          <div className="race-conditions">
            <h4>レース条件</h4>
            <div className="condition-row">
              {race.race_wind !== undefined && (
                <span>風速: {race.race_wind}m</span>
              )}
              {race.race_wave !== undefined && (
                <span>波高: {race.race_wave}cm</span>
              )}
              {race.race_temperature !== undefined && (
                <span>気温: {race.race_temperature}°C</span>
              )}
            </div>
          </div>
        )}

        {/* 展開ボタン */}
        {participants.length > 0 && (
          <button
            onClick={() => setExpanded(!expanded)}
            className="expand-button"
            style={{
              width: "100%",
              padding: "10px",
              marginTop: "10px",
              backgroundColor: "#e3f2fd",
              color: "#1976d2",
              border: "1px solid #90caf9",
              cursor: "pointer",
              borderRadius: "4px",
              fontSize: "14px",
            }}
          >
            {expanded ? "▲ 詳細を隠す" : "▼ 全選手の詳細を見る"}
          </button>
        )}

        {/* 展開時の全選手情報テーブル */}
        {expanded && (
          <div className="participants-table">
            <table>
              <thead>
                <tr>
                  <th>艇</th>
                  <th>選手名</th>
                  <th>級</th>
                  <th>着順</th>
                  <th>コース</th>
                  <th>ST</th>
                  <th>全国勝率</th>
                  <th>モーター</th>
                </tr>
              </thead>
              <tbody>
                {allParticipants.map((p) => (
                  <tr key={p.boat_number}>
                    <td>{p.boat_number}</td>
                    <td>{p.racer_name || "-"}</td>
                    <td>{getRacerClassName(p.racer_class_number)}</td>
                    <td>{p.place_number || "-"}</td>
                    <td>{p.course_number || "-"}</td>
                    <td>{p.start_timing?.toFixed(2) || "-"}</td>
                    <td>{p.national_top_2_percent?.toFixed(1) || "-"}%</td>
                    <td>
                      {p.assigned_motor_number || "-"}
                      {p.assigned_motor_top_2_percent && (
                        <span style={{ fontSize: "11px", color: "#666" }}>
                          ({p.assigned_motor_top_2_percent.toFixed(1)}%)
                        </span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
};

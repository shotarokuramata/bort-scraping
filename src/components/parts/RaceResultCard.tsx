import React from "react";
import { RaceResult } from "../../types/OpenApiData";
import { kyoteiPlaces } from "../../information";

interface RaceResultCardProps {
  result: RaceResult;
  index: number;
}

export const RaceResultCard: React.FC<RaceResultCardProps> = ({
  result,
  index,
}) => {
  // 配当金額を取得
  const getTrifectaPayout = () => {
    return (
      result.payouts.trifecta?.[0]?.payout?.toLocaleString("ja-JP") || "-"
    );
  };

  const getWinPayout = () => {
    return result.payouts.win?.[0]?.payout?.toLocaleString("ja-JP") || "-";
  };

  const getExactaPayout = () => {
    return result.payouts.exacta?.[0]?.payout?.toLocaleString("ja-JP") || "-";
  };

  const getPlacePayout = () => {
    const payouts = result.payouts.place || [];
    if (payouts.length === 0) return "-";
    return payouts
      .map((p) => p.payout?.toLocaleString("ja-JP"))
      .filter((p) => p)
      .join(", ");
  };

  // 勝者情報を取得
  const getWinners = () => {
    return result.boats
      .filter((boat) => boat.racer_place_number && boat.racer_place_number <= 3)
      .sort((a, b) => (a.racer_place_number || 0) - (b.racer_place_number || 0))
      .map((boat) => ({
        place: boat.racer_place_number,
        boat: boat.racer_boat_number,
        name: boat.racer_name || "不明",
      }));
  };

  const winners = getWinners();

  // 日付をフォーマット
  const formatDate = (dateStr: string) => {
    if (dateStr.length !== 8) return dateStr;
    const year = dateStr.slice(0, 4);
    const month = dateStr.slice(4, 6);
    const day = dateStr.slice(6, 8);
    return `${year}/${month}/${day}`;
  };

  // 会場名を取得
  const venueName = kyoteiPlaces[result.race_stadium_number] || `不明(${result.race_stadium_number})`;

  return (
    <div className="race-result-card">
      <div className="card-header">
        <span className="rank">#{index + 1}</span>
        <span className="date">{formatDate(result.race_date)}</span>
        <span className="venue">{venueName}</span>
        <span className="race-number">{result.race_number}R</span>
      </div>

      <div className="card-body">
        <div className="winners">
          <h4>着順</h4>
          {winners.map((w) => (
            <div key={w.place} className="winner-row">
              <span className="place">{w.place}着</span>
              <span className="boat-number">{w.boat}号艇</span>
              <span className="racer-name">{w.name}</span>
            </div>
          ))}
        </div>

        <div className="payouts">
          <h4>配当</h4>
          <div className="payout-row">
            <span className="payout-label">3連単:</span>
            <span className="payout-value">¥{getTrifectaPayout()}</span>
          </div>
          <div className="payout-row">
            <span className="payout-label">2連単:</span>
            <span className="payout-value">¥{getExactaPayout()}</span>
          </div>
          <div className="payout-row">
            <span className="payout-label">単勝:</span>
            <span className="payout-value">¥{getWinPayout()}</span>
          </div>
          <div className="payout-row">
            <span className="payout-label">複勝:</span>
            <span className="payout-value">¥{getPlacePayout()}</span>
          </div>
        </div>

        {result.race_wind !== undefined && (
          <div className="race-conditions">
            <h4>レース条件</h4>
            <div className="condition-row">
              <span>風速: {result.race_wind}m</span>
              {result.race_wave !== undefined && (
                <span>波高: {result.race_wave}cm</span>
              )}
              {result.race_temperature !== undefined && (
                <span>気温: {result.race_temperature}°C</span>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

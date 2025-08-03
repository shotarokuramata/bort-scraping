import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { kyoteiPlaces } from "./information";
import "./App.css";

interface PlayerBasicInfo {
  registration_number: string;
  name: string;
  class_level: string;
  period: string;
  support_group: string;
  gender: string;
}

interface PerformanceData {
  this_period?: number;
  last_6_months?: number;
  last_3_months?: number;
  last_1_month?: number;
  local_venue?: number;
  general_races?: number;
  sg_g1?: number;
}

interface LaneWinRateData {
  last_1_year?: number;
  last_6_months?: number;
}

interface DetailedPerformanceData {
  first_place_rate: PerformanceData;
  top_2_rate: PerformanceData;
  top_3_rate: PerformanceData;
  lane_win_rate: LaneWinRateData;
}

interface STData {
  this_period?: number;
  last_6_months?: number;
  last_3_months?: number;
  last_1_month?: number;
  local_venue?: number;
  general_races?: number;
  sg_g1?: number;
  first_day?: number;
  final_day?: number;
  night_races?: number;
  flying_history?: number;
}

interface STAnalysisData {
  stability_rate?: number;
  break_out_rate?: number;
  late_start_rate?: number;
}

interface STRelatedData {
  average_st: STData;
  st_ranking: STData;
  st_analysis: STAnalysisData;
}

interface WinningHandData {
  escape_rate_6months?: number;
  let_escape_rate_6months?: number;
  pierced_rate_6months?: number;
  pierce_rate_6months?: number;
  overtake_rate_6months?: number;
}

interface OddsCombination {
  first: number;
  second: number;
  third?: number;
  odds: number;
  is_combined: boolean;
  range_text?: string; // 複勝オッズの場合、元の範囲文字列（例："2.4-3.5"）
}

type BettingType = "Trifecta" | "Tricast" | "Exacta" | "Quinella" | "QuinellaPlace" | "WinPlace";

interface OddsData {
  betting_type: BettingType;
  combinations: OddsCombination[];
}

interface RaceData {
  escape_last_year: number;
  escape_last_half_year: number;
  allow_escape_last_year: number;
  allow_escape_last_half_year: number;
  pierce_last_year: number;
  pierce_last_half_year: number;
  overtake_last_year: number;
  overtake_last_half_year: number;
  first_place_in_last_ten_race: number;
  player_basic_info: PlayerBasicInfo;
  detailed_performance: DetailedPerformanceData;
  st_data: STRelatedData;
  winning_hand: WinningHandData;
}

function App() {
  const [raceData, setRaceData] = useState<RaceData | null>(null);
  const [oddsData, setOddsData] = useState<OddsData | null>(null);
  const [winPlaceOddsData, setWinPlaceOddsData] = useState<OddsData | null>(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [oddsLoading, setOddsLoading] = useState(false);
  const [winPlaceOddsLoading, setWinPlaceOddsLoading] = useState(false);
  const [date, setDate] = useState("");
  const [raceNumber, setRaceNumber] = useState("1");
  const [placeNumber, setPlaceNumber] = useState("1");

  async function fetchRaceData() {
    setLoading(true);
    setError("");
    setRaceData(null);
    try {
      const result = await invoke<RaceData>("get_biyori_info", { date, raceNumber, placeNumber });
      setRaceData(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  }

  async function fetchOddsData() {
    setOddsLoading(true);
    setError("");
    setOddsData(null);
    try {
      const result = await invoke<OddsData>("get_parsed_odds_info", { date, raceNumber, placeNumber });
      console.log("オッズデータ取得成功:", result);
      setOddsData(result);
      alert(`オッズデータ取得成功: ${result.combinations.length}個の組み合わせ`);
    } catch (err) {
      console.error("オッズデータ取得エラー:", err);
      setError(err as string);
    } finally {
      setOddsLoading(false);
    }
  }

  async function fetchWinPlaceOddsData() {
    setWinPlaceOddsLoading(true);
    setError("");
    setWinPlaceOddsData(null);
    try {
      const result = await invoke<OddsData>("get_win_place_odds_info", { date, raceNumber, placeNumber });
      console.log("単勝・複勝オッズデータ取得成功:", result);
      setWinPlaceOddsData(result);
      alert(`単勝・複勝オッズデータ取得成功: ${result.combinations.length}個のオッズ`);
    } catch (err) {
      console.error("単勝・複勝オッズデータ取得エラー:", err);
      setError(err as string);
    } finally {
      setWinPlaceOddsLoading(false);
    }
  }

  return (
    <main className="container">
      <h1>Welcome to bort scraping tool !!</h1>

      <div className="main-content">
        <div className="left-panel">
          <form
            className="form"
            onSubmit={(e) => {
              e.preventDefault();
              fetchRaceData();
            }}
          >
            <h2>データ取得</h2>
            <div className="form-group">
              <label htmlFor="date">日付を選択:</label>
              <input
                id="date"
                onChange={(e) => setDate(e.currentTarget.value)}
                placeholder="日付を選択してください"
                type="date"
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="raceNumber">レース番号を選択:</label>
              <select
                id="raceNumber"
                value={raceNumber}
                onChange={(e) => setRaceNumber(e.currentTarget.value)}
              >
                {Array.from({ length: 12 }, (_, i) => i + 1).map((num) => (
                  <option key={num} value={num}>
                    {num}R
                  </option>
                ))}
              </select>
            </div>

            <div className="form-group">
              <label htmlFor="placeNumber">競艇場を選択:</label>
              <select
                id="placeNumber"
                value={placeNumber}
                onChange={(e) => setPlaceNumber(e.currentTarget.value)}
              >
                {Object.entries(kyoteiPlaces).map(([key, name]) => (
                  <option key={key} value={key}>
                    {name}
                  </option>
                ))}
              </select>
            </div>
            
            <button type="submit" disabled={loading || !date}>
              {loading ? "取得中..." : "データ取得"}
            </button>
            
            <button 
              type="button" 
              onClick={fetchOddsData}
              disabled={oddsLoading || !date}
              className="odds-button"
            >
              {oddsLoading ? "三連単取得中..." : "三連単オッズ取得"}
            </button>
            
            <button 
              type="button" 
              onClick={fetchWinPlaceOddsData}
              disabled={winPlaceOddsLoading || !date}
              className="win-place-odds-button"
            >
              {winPlaceOddsLoading ? "単勝・複勝取得中..." : "単勝・複勝オッズ取得"}
            </button>
          </form>
        </div>

        <div className="right-panel">
          {error && <div className="error">{error}</div>}

          {raceData && (
            <div className="race-data">
              {/* 選手基本情報 */}
              <div className="player-info-section">
                <h2>選手基本情報（1号艇）</h2>
                <div className="player-basic-info">
                  <div className="info-grid">
                    <div className="info-item">
                      <span className="info-label">選手名:</span>
                      <span className="info-value">{raceData.player_basic_info.name}</span>
                    </div>
                    <div className="info-item">
                      <span className="info-label">登録番号:</span>
                      <span className="info-value">{raceData.player_basic_info.registration_number}</span>
                    </div>
                    <div className="info-item">
                      <span className="info-label">級別:</span>
                      <span className="info-value">{raceData.player_basic_info.class_level}</span>
                    </div>
                    <div className="info-item">
                      <span className="info-label">期別:</span>
                      <span className="info-value">{raceData.player_basic_info.period}</span>
                    </div>
                    <div className="info-item">
                      <span className="info-label">支部:</span>
                      <span className="info-value">{raceData.player_basic_info.support_group}</span>
                    </div>
                    <div className="info-item">
                      <span className="info-label">性別:</span>
                      <span className="info-value">{raceData.player_basic_info.gender}</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* 基本統計データ */}
              <div className="basic-stats-section">
                <h2>基本統計データ</h2>
                <table className="data-table">
                  <thead>
                    <tr>
                      <th>項目</th>
                      <th>半年間</th>
                      <th>1年間</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td>逃げ率</td>
                      <td>{(raceData.escape_last_half_year * 100).toFixed(1)}%</td>
                      <td>{(raceData.escape_last_year * 100).toFixed(1)}%</td>
                    </tr>
                    <tr>
                      <td>逃がし率</td>
                      <td>{(raceData.allow_escape_last_half_year * 100).toFixed(1)}%</td>
                      <td>{(raceData.allow_escape_last_year * 100).toFixed(1)}%</td>
                    </tr>
                    <tr>
                      <td>差され率</td>
                      <td>{(raceData.pierce_last_half_year * 100).toFixed(1)}%</td>
                      <td>{(raceData.pierce_last_year * 100).toFixed(1)}%</td>
                    </tr>
                    <tr>
                      <td>捲られ率</td>
                      <td>{(raceData.overtake_last_half_year * 100).toFixed(1)}%</td>
                      <td>{(raceData.overtake_last_year * 100).toFixed(1)}%</td>
                    </tr>
                  </tbody>
                </table>
                
                <div className="additional-info">
                  <h3>直近10レースの成績</h3>
                  <div className="stat-item">
                    <span className="stat-label">1着回数:</span>
                    <span className="stat-value">{raceData.first_place_in_last_ten_race}回</span>
                  </div>
                </div>
              </div>

              {/* 詳細成績データ */}
              <div className="performance-section">
                <h2>詳細成績データ</h2>
                
                <div className="performance-grid">
                  {/* 1着率 */}
                  <div className="performance-card">
                    <h3>1着率</h3>
                    <div className="performance-data">
                      {raceData.detailed_performance.first_place_rate.this_period !== undefined && (
                        <div className="perf-item">
                          <span>今期:</span> {(raceData.detailed_performance.first_place_rate.this_period * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.first_place_rate.last_6_months !== undefined && (
                        <div className="perf-item">
                          <span>直近6ヶ月:</span> {(raceData.detailed_performance.first_place_rate.last_6_months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.first_place_rate.last_3_months !== undefined && (
                        <div className="perf-item">
                          <span>直近3ヶ月:</span> {(raceData.detailed_performance.first_place_rate.last_3_months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.first_place_rate.last_1_month !== undefined && (
                        <div className="perf-item">
                          <span>直近1ヶ月:</span> {(raceData.detailed_performance.first_place_rate.last_1_month * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.first_place_rate.local_venue !== undefined && (
                        <div className="perf-item">
                          <span>当地:</span> {(raceData.detailed_performance.first_place_rate.local_venue * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.first_place_rate.general_races !== undefined && (
                        <div className="perf-item">
                          <span>一般戦:</span> {(raceData.detailed_performance.first_place_rate.general_races * 100).toFixed(1)}%
                        </div>
                      )}
                    </div>
                  </div>

                  {/* 2連対率 */}
                  <div className="performance-card">
                    <h3>2連対率</h3>
                    <div className="performance-data">
                      {raceData.detailed_performance.top_2_rate.this_period !== undefined && (
                        <div className="perf-item">
                          <span>今期:</span> {(raceData.detailed_performance.top_2_rate.this_period * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_2_rate.last_6_months !== undefined && (
                        <div className="perf-item">
                          <span>直近6ヶ月:</span> {(raceData.detailed_performance.top_2_rate.last_6_months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_2_rate.last_3_months !== undefined && (
                        <div className="perf-item">
                          <span>直近3ヶ月:</span> {(raceData.detailed_performance.top_2_rate.last_3_months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_2_rate.last_1_month !== undefined && (
                        <div className="perf-item">
                          <span>直近1ヶ月:</span> {(raceData.detailed_performance.top_2_rate.last_1_month * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_2_rate.local_venue !== undefined && (
                        <div className="perf-item">
                          <span>当地:</span> {(raceData.detailed_performance.top_2_rate.local_venue * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_2_rate.general_races !== undefined && (
                        <div className="perf-item">
                          <span>一般戦:</span> {(raceData.detailed_performance.top_2_rate.general_races * 100).toFixed(1)}%
                        </div>
                      )}
                    </div>
                  </div>

                  {/* 3連対率 */}
                  <div className="performance-card">
                    <h3>3連対率</h3>
                    <div className="performance-data">
                      {raceData.detailed_performance.top_3_rate.this_period !== undefined && (
                        <div className="perf-item">
                          <span>今期:</span> {(raceData.detailed_performance.top_3_rate.this_period * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_3_rate.last_6_months !== undefined && (
                        <div className="perf-item">
                          <span>直近6ヶ月:</span> {(raceData.detailed_performance.top_3_rate.last_6_months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_3_rate.last_3_months !== undefined && (
                        <div className="perf-item">
                          <span>直近3ヶ月:</span> {(raceData.detailed_performance.top_3_rate.last_3_months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_3_rate.last_1_month !== undefined && (
                        <div className="perf-item">
                          <span>直近1ヶ月:</span> {(raceData.detailed_performance.top_3_rate.last_1_month * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_3_rate.local_venue !== undefined && (
                        <div className="perf-item">
                          <span>当地:</span> {(raceData.detailed_performance.top_3_rate.local_venue * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.detailed_performance.top_3_rate.general_races !== undefined && (
                        <div className="perf-item">
                          <span>一般戦:</span> {(raceData.detailed_performance.top_3_rate.general_races * 100).toFixed(1)}%
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>

              {/* ST関連データ */}
              <div className="st-section">
                <h2>ST関連データ</h2>
                
                <div className="st-grid">
                  {/* 平均ST */}
                  <div className="st-card">
                    <h3>平均ST</h3>
                    <div className="st-data">
                      {raceData.st_data.average_st.this_period !== undefined && (
                        <div className="st-item">
                          <span>今期:</span> {raceData.st_data.average_st.this_period.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.average_st.last_6_months !== undefined && (
                        <div className="st-item">
                          <span>直近6ヶ月:</span> {raceData.st_data.average_st.last_6_months.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.average_st.last_3_months !== undefined && (
                        <div className="st-item">
                          <span>直近3ヶ月:</span> {raceData.st_data.average_st.last_3_months.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.average_st.local_venue !== undefined && (
                        <div className="st-item">
                          <span>当地:</span> {raceData.st_data.average_st.local_venue.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.average_st.general_races !== undefined && (
                        <div className="st-item">
                          <span>一般戦:</span> {raceData.st_data.average_st.general_races.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.average_st.night_races !== undefined && (
                        <div className="st-item">
                          <span>ナイター:</span> {raceData.st_data.average_st.night_races.toFixed(2)}
                        </div>
                      )}
                    </div>
                  </div>

                  {/* ST順位 */}
                  <div className="st-card">
                    <h3>ST順位</h3>
                    <div className="st-data">
                      {raceData.st_data.st_ranking.this_period !== undefined && (
                        <div className="st-item">
                          <span>今期:</span> {raceData.st_data.st_ranking.this_period.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.st_ranking.last_6_months !== undefined && (
                        <div className="st-item">
                          <span>直近6ヶ月:</span> {raceData.st_data.st_ranking.last_6_months.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.st_ranking.last_3_months !== undefined && (
                        <div className="st-item">
                          <span>直近3ヶ月:</span> {raceData.st_data.st_ranking.last_3_months.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.st_ranking.local_venue !== undefined && (
                        <div className="st-item">
                          <span>当地:</span> {raceData.st_data.st_ranking.local_venue.toFixed(2)}
                        </div>
                      )}
                      {raceData.st_data.st_ranking.general_races !== undefined && (
                        <div className="st-item">
                          <span>一般戦:</span> {raceData.st_data.st_ranking.general_races.toFixed(2)}
                        </div>
                      )}
                    </div>
                  </div>

                  {/* ST考察 */}
                  <div className="st-card">
                    <h3>ST考察</h3>
                    <div className="st-data">
                      {raceData.st_data.st_analysis.stability_rate !== undefined && (
                        <div className="st-item">
                          <span>安定率:</span> {(raceData.st_data.st_analysis.stability_rate * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.st_data.st_analysis.break_out_rate !== undefined && (
                        <div className="st-item">
                          <span>抜出率:</span> {(raceData.st_data.st_analysis.break_out_rate * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.st_data.st_analysis.late_start_rate !== undefined && (
                        <div className="st-item">
                          <span>出遅率:</span> {(raceData.st_data.st_analysis.late_start_rate * 100).toFixed(1)}%
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>

              {/* 決まり手データ */}
              <div className="winning-hand-section">
                <h2>決まり手データ（直近6ヶ月）</h2>
                
                <div className="winning-hand-grid">
                  {/* 1号艇の決まり手 */}
                  <div className="winning-hand-card">
                    <h3>1号艇の決まり手</h3>
                    <div className="winning-hand-data">
                      {raceData.winning_hand.escape_rate_6months !== undefined && (
                        <div className="winning-hand-item">
                          <span>逃げ率:</span> {(raceData.winning_hand.escape_rate_6months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.winning_hand.pierced_rate_6months !== undefined && (
                        <div className="winning-hand-item">
                          <span>差され率:</span> {(raceData.winning_hand.pierced_rate_6months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.winning_hand.overtake_rate_6months !== undefined && (
                        <div className="winning-hand-item">
                          <span>捲られ率:</span> {(raceData.winning_hand.overtake_rate_6months * 100).toFixed(1)}%
                        </div>
                      )}
                    </div>
                  </div>

                  {/* 2号艇以降の決まり手 */}
                  <div className="winning-hand-card">
                    <h3>2号艇以降の決まり手</h3>
                    <div className="winning-hand-data">
                      {raceData.winning_hand.pierce_rate_6months !== undefined && (
                        <div className="winning-hand-item">
                          <span>2号艇差し率:</span> {(raceData.winning_hand.pierce_rate_6months * 100).toFixed(1)}%
                        </div>
                      )}
                      {raceData.winning_hand.let_escape_rate_6months !== undefined && (
                        <div className="winning-hand-item">
                          <span>逃し率:</span> {(raceData.winning_hand.let_escape_rate_6months * 100).toFixed(1)}%
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>

            </div>
          )}

          {/* オッズデータ（独立表示） */}
          {oddsData && (
            <div className="odds-section">
              <h2>オッズデータ（{oddsData.betting_type === "Trifecta" ? "三連単" : oddsData.betting_type}）</h2>
              
              <div className="odds-summary">
                <p>総組み合わせ数: {oddsData.combinations.length}通り</p>
              </div>
              
              <div className="odds-grid">
                {/* 人気上位10組み合わせ（オッズが低い順） */}
                <div className="odds-card">
                  <h3>人気上位10組み合わせ</h3>
                  <div className="odds-list">
                    {oddsData.combinations
                      .sort((a, b) => a.odds - b.odds)
                      .slice(0, 10)
                      .map((combo, index) => (
                        <div key={index} className="odds-item">
                          <span className="combination">
                            {combo.third ? 
                              `${combo.first}-${combo.second}-${combo.third}` : 
                              `${combo.first}-${combo.second}`
                            }
                          </span>
                          <span className="odds-value">{combo.odds}</span>
                          {combo.is_combined && <span className="combined-badge">合成</span>}
                        </div>
                      ))}
                  </div>
                </div>

                {/* 穴馬上位10組み合わせ（オッズが高い順） */}
                <div className="odds-card">
                  <h3>穴馬上位10組み合わせ</h3>
                  <div className="odds-list">
                    {oddsData.combinations
                      .sort((a, b) => b.odds - a.odds)
                      .slice(0, 10)
                      .map((combo, index) => (
                        <div key={index} className="odds-item">
                          <span className="combination">
                            {combo.third ? 
                              `${combo.first}-${combo.second}-${combo.third}` : 
                              `${combo.first}-${combo.second}`
                            }
                          </span>
                          <span className="odds-value">{combo.odds}</span>
                          {combo.is_combined && <span className="combined-badge">合成</span>}
                        </div>
                      ))}
                  </div>
                </div>

                {/* 1号艇関連組み合わせ */}
                <div className="odds-card">
                  <h3>1号艇関連組み合わせ</h3>
                  <div className="odds-list">
                    {oddsData.combinations
                      .filter(combo => combo.first === 1)
                      .sort((a, b) => a.odds - b.odds)
                      .slice(0, 10)
                      .map((combo, index) => (
                        <div key={index} className="odds-item">
                          <span className="combination">
                            {combo.third ? 
                              `${combo.first}-${combo.second}-${combo.third}` : 
                              `${combo.first}-${combo.second}`
                            }
                          </span>
                          <span className="odds-value">{combo.odds}</span>
                          {combo.is_combined && <span className="combined-badge">合成</span>}
                        </div>
                      ))}
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* 単勝・複勝オッズデータ（独立表示） */}
          {winPlaceOddsData && (
            <div className="win-place-odds-section">
              <h2>単勝・複勝オッズデータ</h2>
              
              <div className="win-place-odds-summary">
                <p>総オッズ数: {winPlaceOddsData.combinations.length}個</p>
              </div>
              
              <div className="win-place-odds-grid">
                {/* 単勝オッズ */}
                <div className="win-place-odds-card">
                  <h3>単勝オッズ</h3>
                  <div className="win-place-odds-list">
                    {winPlaceOddsData.combinations
                      .filter(combo => combo.second === 0) // 単勝オッズ
                      .sort((a, b) => a.first - b.first) // 艇番順でソート
                      .map((combo, index) => (
                        <div key={index} className="win-place-odds-item">
                          <span className="boat-number">{combo.first}号艇</span>
                          <span className="odds-value">{combo.odds.toFixed(1)}倍</span>
                        </div>
                      ))}
                  </div>
                </div>

                {/* 複勝オッズ */}
                <div className="win-place-odds-card">
                  <h3>複勝オッズ</h3>
                  <div className="win-place-odds-list">
                    {winPlaceOddsData.combinations
                      .filter(combo => combo.second === 1) // 複勝オッズ
                      .sort((a, b) => a.first - b.first) // 艇番順でソート
                      .map((combo, index) => (
                        <div key={index} className="win-place-odds-item">
                          <span className="boat-number">{combo.first}号艇</span>
                          <span className="odds-value">{combo.range_text || combo.odds.toFixed(1)}倍</span>
                        </div>
                      ))}
                  </div>
                </div>
              </div>
            </div>
          )}

          {!raceData && !error && !loading && !oddsData && !oddsLoading && !winPlaceOddsData && !winPlaceOddsLoading && (
            <div className="placeholder">
              <p>左のフォームからレース情報を取得してください</p>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}

export default App;

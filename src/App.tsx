import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { kyoteiPlaces } from "./information";
import "./App.css";

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
}

function App() {
  const [raceData, setRaceData] = useState<RaceData | null>(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
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
          </form>
        </div>

        <div className="right-panel">
          {error && <div className="error">{error}</div>}

          {raceData && (
            <div className="race-data">
              <h2>レース統計データ</h2>
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
          )}

          {!raceData && !error && !loading && (
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

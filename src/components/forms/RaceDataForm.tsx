import { kyoteiPlaces } from "../../information";

interface RaceDataFormProps {
  date: string;
  raceNumber: string;
  placeNumber: string;
  loading: boolean;
  winPlaceOddsLoading: boolean;
  onDateChange: (date: string) => void;
  onRaceNumberChange: (raceNumber: string) => void;
  onPlaceNumberChange: (placeNumber: string) => void;
  onSubmit: () => void;
  onWinPlaceOddsClick: () => void;
}

export function RaceDataForm({
  date,
  raceNumber,
  placeNumber,
  loading,
  winPlaceOddsLoading,
  onDateChange,
  onRaceNumberChange,
  onPlaceNumberChange,
  onSubmit,
  onWinPlaceOddsClick,
}: RaceDataFormProps) {
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit();
  };

  return (
    <form className="form" onSubmit={handleSubmit}>
      <h2>データ取得</h2>
      <div className="form-group">
        <label htmlFor="date">日付を選択:</label>
        <input
          id="date"
          onChange={(e) => onDateChange(e.currentTarget.value)}
          placeholder="日付を選択してください"
          type="date"
          value={date}
          required
        />
      </div>

      <div className="form-group">
        <label htmlFor="raceNumber">レース番号を選択:</label>
        <select
          id="raceNumber"
          value={raceNumber}
          onChange={(e) => onRaceNumberChange(e.currentTarget.value)}
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
          onChange={(e) => onPlaceNumberChange(e.currentTarget.value)}
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
        onClick={onWinPlaceOddsClick}
        disabled={winPlaceOddsLoading || !date}
        className="win-place-odds-button"
      >
        {winPlaceOddsLoading ? "単勝・複勝取得中..." : "単勝・複勝オッズ取得"}
      </button>
    </form>
  );
}
import { kyoteiPlaces } from "../../information";

interface BulkDataFormProps {
  startDate: string;
  endDate: string;
  selectedPlaces: number[];
  selectedRaces: number[];
  bulkLoading: boolean;
  onStartDateChange: (date: string) => void;
  onEndDateChange: (date: string) => void;
  onPlaceSelectionChange: (placeNumber: number, checked: boolean) => void;
  onRaceSelectionChange: (raceNumber: number, checked: boolean) => void;
  onSubmit: () => void;
}

export function BulkDataForm({
  startDate,
  endDate,
  selectedPlaces,
  selectedRaces,
  bulkLoading,
  onStartDateChange,
  onEndDateChange,
  onPlaceSelectionChange,
  onRaceSelectionChange,
  onSubmit,
}: BulkDataFormProps) {
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit();
  };

  return (
    <form className="form bulk-form" onSubmit={handleSubmit}>
      <h2>一括データ取得</h2>
      
      <div className="form-group">
        <label htmlFor="startDate">開始日:</label>
        <input
          id="startDate"
          type="date"
          value={startDate}
          onChange={(e) => onStartDateChange(e.target.value)}
          required
        />
      </div>

      <div className="form-group">
        <label htmlFor="endDate">終了日:</label>
        <input
          id="endDate"
          type="date"
          value={endDate}
          onChange={(e) => onEndDateChange(e.target.value)}
          required
        />
      </div>

      <div className="form-group">
        <label>競艇場を選択:</label>
        <div className="checkbox-group">
          {Object.entries(kyoteiPlaces).slice(0, 12).map(([key, name]) => (
            <label key={key} className="checkbox-label">
              <input
                type="checkbox"
                checked={selectedPlaces.includes(parseInt(key))}
                onChange={(e) => {
                  onPlaceSelectionChange(parseInt(key), e.target.checked);
                }}
              />
              {name}
            </label>
          ))}
        </div>
      </div>

      <div className="form-group">
        <label>レース番号を選択:</label>
        <div className="checkbox-group">
          {Array.from({ length: 12 }, (_, i) => i + 1).map((num) => (
            <label key={num} className="checkbox-label">
              <input
                type="checkbox"
                checked={selectedRaces.includes(num)}
                onChange={(e) => {
                  onRaceSelectionChange(num, e.target.checked);
                }}
              />
              {num}R
            </label>
          ))}
        </div>
      </div>

      <button type="submit" disabled={bulkLoading || !startDate || !endDate}>
        {bulkLoading ? "一括取得中..." : "一括取得開始"}
      </button>
    </form>
  );
}
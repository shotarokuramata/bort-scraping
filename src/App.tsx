import { useState } from "react";
import { HelloMessage, RaceDataForm, BulkDataForm } from "./components/forms";
import { RaceDataContainer, OddsDataContainer, BulkResultsContainer, ActiveRacesTable } from "./components/containers";
import { useHelloWorld, useActiveRaces, useRaceData, useOddsData, useBulkData } from "./hooks";
import "./App.css";

function App() {
  const helloMessage = useHelloWorld();
  const { activeRaces, loading: activeRacesLoading, error: activeRacesError } = useActiveRaces();
  const { raceData, loading: raceLoading, error: raceError, fetchRaceData } = useRaceData();
  const { oddsData, loading: oddsLoading, error: oddsError, fetchOddsData } = useOddsData();
  const { bulkData, loading: bulkLoading, error: bulkError, fetchBulkData } = useBulkData();

  // フォーム状態管理
  const [date, setDate] = useState("");
  const [raceNumber, setRaceNumber] = useState("1");
  const [placeNumber, setPlaceNumber] = useState("1");
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");
  const [selectedPlaces, setSelectedPlaces] = useState<number[]>([1]);
  const [selectedRaces, setSelectedRaces] = useState<number[]>([1]);

  // 統合エラー表示
  const error = activeRacesError || raceError || oddsError || bulkError;

  const handlePlaceSelectionChange = (placeNumber: number, checked: boolean) => {
    if (checked) {
      setSelectedPlaces([...selectedPlaces, placeNumber]);
    } else {
      setSelectedPlaces(selectedPlaces.filter(p => p !== placeNumber));
    }
  };

  const handleRaceSelectionChange = (raceNumber: number, checked: boolean) => {
    if (checked) {
      setSelectedRaces([...selectedRaces, raceNumber]);
    } else {
      setSelectedRaces(selectedRaces.filter(r => r !== raceNumber));
    }
  };

  const handleActiveRaceSelect = (placeId: number, _placeName: string, raceNumber: number) => {
    // 現在の日付を設定
    const today = new Date().toISOString().split('T')[0];
    setDate(today);
    setPlaceNumber(placeId.toString());
    setRaceNumber(raceNumber.toString());
    
    // 自動でデータを取得
    fetchRaceData(today, raceNumber.toString(), placeId.toString());
    fetchOddsData(today, raceNumber.toString(), placeId.toString());
  };

  return (
    <main className="container">
      <h1>Welcome to bort scraping tool !!</h1>
      <HelloMessage message={helloMessage} />

      {activeRacesLoading && <div>開催レース場を読み込み中...</div>}
      
      <ActiveRacesTable 
        activeRaces={activeRaces}
        onRaceSelect={handleActiveRaceSelect}
      />

      <div className="main-content">
        <div className="left-panel">
          <RaceDataForm
            date={date}
            raceNumber={raceNumber}
            placeNumber={placeNumber}
            loading={raceLoading}
            winPlaceOddsLoading={oddsLoading}
            onDateChange={setDate}
            onRaceNumberChange={setRaceNumber}
            onPlaceNumberChange={setPlaceNumber}
            onSubmit={() => fetchRaceData(date, raceNumber, placeNumber)}
            onWinPlaceOddsClick={() => fetchOddsData(date, raceNumber, placeNumber)}
          />

          <BulkDataForm
            startDate={startDate}
            endDate={endDate}
            selectedPlaces={selectedPlaces}
            selectedRaces={selectedRaces}
            bulkLoading={bulkLoading}
            onStartDateChange={setStartDate}
            onEndDateChange={setEndDate}
            onPlaceSelectionChange={handlePlaceSelectionChange}
            onRaceSelectionChange={handleRaceSelectionChange}
            onSubmit={() => fetchBulkData(startDate, endDate, selectedPlaces, selectedRaces)}
          />
        </div>

        <div className="right-panel">
          {error && <div className="error">{error}</div>}

          {/* レースデータローディング表示 */}
          {raceLoading && (
            <div className="loading-message">
              <div className="loading-spinner"></div>
              <p>レースデータを取得中...</p>
            </div>
          )}
          <RaceDataContainer raceData={raceData} />

          {/* オッズデータローディング表示 */}
          {oddsLoading && (
            <div className="loading-message">
              <div className="loading-spinner"></div>
              <p>オッズデータを取得中...</p>
            </div>
          )}
          <OddsDataContainer oddsData={oddsData} />

          {/* 一括取得ローディング表示 */}
          {bulkLoading && (
            <div className="loading-message">
              <div className="loading-spinner"></div>
              <p>一括データを取得中...</p>
            </div>
          )}
          <BulkResultsContainer bulkData={bulkData} />

          {!raceData && !error && !raceLoading && !oddsData && !oddsLoading && bulkData.length === 0 && (
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

import { useState } from "react";
import { HelloMessage, RaceDataForm, BulkDataForm } from "./components/forms";
import { RaceDataContainer, OddsDataContainer, BulkResultsContainer } from "./components/containers";
import { useHelloWorld, useRaceData, useOddsData, useBulkData } from "./hooks";
import "./App.css";

function App() {
  const helloMessage = useHelloWorld();
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
  const error = raceError || oddsError || bulkError;

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

  return (
    <main className="container">
      <h1>Welcome to bort scraping tool !!</h1>
      <HelloMessage message={helloMessage} />

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

          <RaceDataContainer raceData={raceData} />

          <OddsDataContainer oddsData={oddsData} />

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

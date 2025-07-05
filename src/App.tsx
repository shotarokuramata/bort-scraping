import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { kyoteiPlaces } from "./information";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [date, setDate] = useState("");
  const [raceNumber, setRaceNumber] = useState("1");
  const [placeNumber, setPlaceNumber] = useState("1");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("get_biyori_info", { date, raceNumber, placeNumber }));
  }

  return (
    <main className="container">
      <h1>Welcome to bort scraping tool !!</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="date"
          onChange={(e) => setDate(e.currentTarget.value)}
          placeholder="Enter a name..."
          type="date"
        />

<label htmlFor="raceNumber">レース番号を選択:</label>
        <select
          id="raceNumber"
          value={raceNumber}
          onChange={(e) => setRaceNumber(e.currentTarget.value)}
        >
          {Array.from({ length: 12 }, (_, i) => i + 1).map((num) => (
            <option key={num} value={num}>
              {num}
            </option>
          ))}
        </select>

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
        <button type="submit">submit</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;

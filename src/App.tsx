import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [date, setDate] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("get_body", { date }));
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
          id="greet-input"
          onChange={(e) => setDate(e.currentTarget.value)}
          placeholder="Enter a name..."
          type="date"
        />
        <button type="submit">submit</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;

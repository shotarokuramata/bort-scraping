import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Home from "./pages/Home";
import ScrapingTool from "./pages/ScrapingTool";
import OpenApiTool from "./pages/OpenApiTool";
import HighPayoutSearch from "./pages/HighPayoutSearch";
import "./App.css";

function App() {
  const [isInitialized, setIsInitialized] = useState(false);
  const [initError, setInitError] = useState<string | null>(null);

  useEffect(() => {
    const initServices = async () => {
      try {
        await invoke("init_open_api_service");
        console.log("Open API service initialized");
        setIsInitialized(true);
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        console.error("Failed to initialize Open API service:", errorMessage);
        setInitError(errorMessage);
      }
    };

    initServices();
  }, []);

  if (initError) {
    return (
      <div style={{ padding: "2rem", textAlign: "center" }}>
        <h1>初期化エラー</h1>
        <p>サービスの初期化に失敗しました: {initError}</p>
      </div>
    );
  }

  if (!isInitialized) {
    return (
      <div style={{ padding: "2rem", textAlign: "center" }}>
        <h1>初期化中...</h1>
        <p>データベースを準備しています</p>
      </div>
    );
  }

  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/scraping" element={<ScrapingTool />} />
        <Route path="/open-api" element={<OpenApiTool />} />
        <Route path="/high-payout-search" element={<HighPayoutSearch />} />
      </Routes>
    </Router>
  );
}

export default App;

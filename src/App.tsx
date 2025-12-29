import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import ScrapingTool from "./pages/ScrapingTool";
import OpenApiTool from "./pages/OpenApiTool";
import "./App.css";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/scraping" element={<ScrapingTool />} />
        <Route path="/open-api" element={<OpenApiTool />} />
      </Routes>
    </Router>
  );
}

export default App;

import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import ScrapingTool from "./pages/ScrapingTool";
import TableParser from "./pages/TableParser";
import "./App.css";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/scraping" element={<ScrapingTool />} />
        <Route path="/table-parser" element={<TableParser />} />
      </Routes>
    </Router>
  );
}

export default App;

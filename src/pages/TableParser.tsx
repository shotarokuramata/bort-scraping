import { useState } from "react";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { ParsedTableData } from "../types";
import "./TableParser.css";

function TableParser() {
  const [inputData, setInputData] = useState("");
  const [url, setUrl] = useState("");
  const [parsedData, setParsedData] = useState<ParsedTableData | null>(null);
  const [loading, setLoading] = useState(false);
  const [scraping, setScraping] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleScrape = async () => {
    setScraping(true);
    setError(null);

    try {
      // Tauriコマンドを呼び出してURLからHTMLをスクレイピング
      const html = await invoke<string>("scrape_html_from_url", {
        url: url
      });
      setInputData(html);
    } catch (err) {
      setError(err instanceof Error ? err.message : "スクレイピングエラーが発生しました");
    } finally {
      setScraping(false);
    }
  };

  const handleParse = async () => {
    setLoading(true);
    setError(null);

    try {
      // Tauriコマンドを呼び出してテーブルをパース
      const result = await invoke<ParsedTableData>("parse_table", {
        inputData: inputData
      });
      setParsedData(result);

    } catch (err) {
      setError(err instanceof Error ? err.message : "エラーが発生しました");
    } finally {
      setLoading(false);
    }
  };

  const handleClear = () => {
    setInputData("");
    setUrl("");
    setParsedData(null);
    setError(null);
  };

  return (
    <main className="table-parser-container">
      <div className="header">
        <h1>テーブルパーサー</h1>
        <Link to="/" className="home-button">
          ホームに戻る
        </Link>
      </div>

      <div className="parser-content">
        <div className="input-section">
          <h2>入力</h2>

          {/* URL入力セクション */}
          <div className="url-input-section">
            <h3>URLからスクレイピング</h3>
            <input
              type="text"
              className="url-input"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder="https://example.com"
              disabled={scraping}
            />
            <button
              className="scrape-button"
              onClick={handleScrape}
              disabled={scraping || !url}
            >
              {scraping ? "スクレイピング中..." : "HTMLを取得"}
            </button>
          </div>

          {/* テキスト入力セクション */}
          <div className="text-input-section">
            <h3>または直接入力</h3>
            <textarea
              className="input-textarea"
              value={inputData}
              onChange={(e) => setInputData(e.target.value)}
              placeholder="ここにデータを入力してください..."
              rows={15}
              disabled={scraping}
            />
          </div>

          <div className="button-group">
            <button
              className="parse-button"
              onClick={handleParse}
              disabled={loading || scraping || !inputData}
            >
              {loading ? "処理中..." : "パース実行"}
            </button>
            <button
              className="clear-button"
              onClick={handleClear}
              disabled={loading || scraping}
            >
              クリア
            </button>
          </div>
        </div>

        <div className="output-section">
          <h2>結果</h2>

          {error && (
            <div className="error-message">
              <strong>エラー:</strong> {error}
            </div>
          )}

          {scraping && (
            <div className="loading-message">
              <div className="loading-spinner"></div>
              <p>HTMLをスクレイピング中...</p>
            </div>
          )}

          {loading && (
            <div className="loading-message">
              <div className="loading-spinner"></div>
              <p>データを解析中...</p>
            </div>
          )}

          {parsedData && !loading && (
            <div className="result-container">
              <div className="result-summary">
                <h3>{parsedData.summary}</h3>
                <p>行数: {parsedData.line_count} | 文字数: {parsedData.char_count}</p>
              </div>
              <pre className="result-display">
                {parsedData.data.join('\n')}
              </pre>
            </div>
          )}

          {!parsedData && !loading && !error && (
            <div className="placeholder">
              <p>パース結果がここに表示されます</p>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}

export default TableParser;

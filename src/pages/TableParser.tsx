import { useState } from "react";
import { Link } from "react-router-dom";
import "./TableParser.css";

function TableParser() {
  const [inputData, setInputData] = useState("");
  const [parsedData, setParsedData] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleParse = async () => {
    setLoading(true);
    setError(null);

    try {
      // TODO: ここにテーブルパース処理を実装
      // 例: Tauriコマンドを呼び出す
      // const result = await invoke("parse_table", { data: inputData });

      // 仮の処理
      await new Promise(resolve => setTimeout(resolve, 1000));
      setParsedData({ message: "パース処理はまだ実装されていません" });

    } catch (err) {
      setError(err instanceof Error ? err.message : "エラーが発生しました");
    } finally {
      setLoading(false);
    }
  };

  const handleClear = () => {
    setInputData("");
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
          <textarea
            className="input-textarea"
            value={inputData}
            onChange={(e) => setInputData(e.target.value)}
            placeholder="ここにデータを入力してください..."
            rows={15}
          />

          <div className="button-group">
            <button
              className="parse-button"
              onClick={handleParse}
              disabled={loading || !inputData}
            >
              {loading ? "処理中..." : "パース実行"}
            </button>
            <button
              className="clear-button"
              onClick={handleClear}
              disabled={loading}
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

          {loading && (
            <div className="loading-message">
              <div className="loading-spinner"></div>
              <p>処理中...</p>
            </div>
          )}

          {parsedData && !loading && (
            <div className="result-container">
              <pre className="result-display">
                {JSON.stringify(parsedData, null, 2)}
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

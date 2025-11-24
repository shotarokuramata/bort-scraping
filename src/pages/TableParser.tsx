import { useState } from "react";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { ParsedTableData, TableWithHeaderAndValues } from "../types";
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

  // テーブルデータを縦持ち（転置）形式に変換
  const transposeTable = (table: TableWithHeaderAndValues): string[][] => {
    const result: string[][] = [];

    // 最初の列（ヘッダーの最初の要素）は単独で出力
    if (table.headers.length > 0) {
      result.push([table.headers[0]]);
    }

    // 2列目以降を転置して出力
    for (let colIndex = 1; colIndex < table.headers.length; colIndex++) {
      const row: string[] = [table.headers[colIndex]];

      // 各行の同じ列の値を追加
      for (const dataRow of table.rows) {
        if (colIndex < dataRow.length) {
          row.push(dataRow[colIndex]);
        }
      }

      result.push(row);
    }

    return result;
  };

  // TSV形式に変換（Excelに貼り付け可能）
  const convertToTSV = (tables: TableWithHeaderAndValues[]): string => {
    const lines: string[] = [];

    for (const table of tables) {
      const transposed = transposeTable(table);

      // 各行をタブ区切りで結合
      for (const row of transposed) {
        lines.push(row.join('\t'));
      }

      // テーブル間に空行を追加
      lines.push('');
    }

    return lines.join('\n');
  };

  // JSON文字列をパースしてテーブルデータに変換
  const parseTableData = (data: string[]): TableWithHeaderAndValues[] => {
    return data.map(jsonStr => JSON.parse(jsonStr) as TableWithHeaderAndValues);
  };

  // クリップボードにコピー
  const handleCopyToClipboard = async () => {
    if (!parsedData) return;

    try {
      const tables = parseTableData(parsedData.data);
      const tsvData = convertToTSV(tables);
      await navigator.clipboard.writeText(tsvData);
      alert("コピーしました！Excelに貼り付けてください。");
    } catch (err) {
      setError("クリップボードへのコピーに失敗しました");
    }
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

          {parsedData && !loading && (() => {
            const tables = parseTableData(parsedData.data);
            const tsvData = convertToTSV(tables);

            return (
              <div className="result-container">
                <div className="result-summary">
                  <h3>{parsedData.summary}</h3>
                  <p>行数: {parsedData.line_count} | 文字数: {parsedData.char_count}</p>
                  <button
                    className="copy-button"
                    onClick={handleCopyToClipboard}
                  >
                    📋 Excelにコピー
                  </button>
                </div>

                {/* テーブル形式のプレビュー（縦持ち） */}
                {tables.map((table, tableIndex) => {
                  const transposed = transposeTable(table);

                  return (
                    <div key={tableIndex} className="table-preview">
                      <h4>テーブル {tableIndex + 1} - プレビュー（縦持ち）</h4>
                      <div className="table-wrapper">
                        <table className="data-table">
                          <tbody>
                            {transposed.map((row, rowIndex) => (
                              <tr key={rowIndex}>
                                {row.map((cell, cellIndex) => (
                                  <td key={cellIndex}>{cell}</td>
                                ))}
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    </div>
                  );
                })}

                {/* TSVテキスト表示 */}
                <div className="tsv-preview">
                  <h4>TSV形式（タブ区切り）</h4>
                  <pre className="result-display">
                    {tsvData}
                  </pre>
                </div>
              </div>
            );
          })()}

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

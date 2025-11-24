use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

/// テーブルパース結果を格納する構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedTableData {
    /// 入力データの行数
    pub line_count: usize,
    /// 入力データの文字数
    pub char_count: usize,
    /// パース結果のサマリー
    pub summary: String,
    /// パース後のデータ（仮実装ではそのまま返す）
    pub data: Vec<String>,
}

/// テーブルのヘッダーとデータ行を格納する構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableWithHeaderAndValues {
    /// ヘッダー行（項目名）
    pub headers: Vec<String>,
    /// データ行（各行が列の値の配列）
    pub rows: Vec<Vec<String>>,
}

/// テーブルデータをパースする関数
///
/// # Arguments
/// * `input` - 入力HTMLデータ
///
/// # Returns
/// パース結果を含む `ParsedTableData`
pub fn parse_table_data(input: &str) -> Result<ParsedTableData, String> {
    if input.trim().is_empty() {
        return Err("入力データが空です".to_string());
    }

    let tables = get_tables_from_html_string(input)?;
    let parsed_tables = parse_tables_header_and_values(&tables)?;

    let line_count = input.lines().count();
    let char_count = input.len();

    // パースしたテーブルデータをJSON文字列として格納
    let data: Vec<String> = parsed_tables
        .iter()
        .enumerate()
        .map(|(i, table)| {
            serde_json::to_string_pretty(table)
                .unwrap_or_else(|_| format!("Error serializing table {}", i))
        })
        .collect();

    let summary = format!(
        "パース完了: {} テーブル, {} 行, {} 文字",
        parsed_tables.len(),
        line_count,
        char_count
    );

    Ok(ParsedTableData {
        line_count,
        char_count,
        summary,
        data,
    })
}

fn get_tables_from_html_string(str: &str) -> Result<Vec<String>, String> {
    let html = Html::parse_document(str);
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let tables: Vec<String> = html
        .select(&table_selector)
        .map(|element| element.html())
        .collect();
    if tables.is_empty() {
        return Err("table not found".to_string());
    }

    return Ok(tables);
}

/// テーブルをrowspan/colspanを考慮して正規化されたグリッドに変換
fn normalize_table_to_grid(rows: &[scraper::ElementRef]) -> Vec<Vec<String>> {
    let cell_selector = Selector::parse("td, th").unwrap();

    // グリッドの最大列数を予測（最初のパスで計算）
    let mut max_cols = 0;
    let mut grid: Vec<Vec<Option<String>>> = Vec::new();

    for (row_idx, row) in rows.iter().enumerate() {
        // 新しい行を追加（必要に応じて拡張）
        while grid.len() <= row_idx {
            grid.push(Vec::new());
        }

        let mut col_idx = 0;

        for cell in row.select(&cell_selector) {
            // 既に埋まっているセルをスキップ
            while col_idx < grid[row_idx].len() && grid[row_idx][col_idx].is_some() {
                col_idx += 1;
            }

            // colspan/rowspanを取得
            let colspan = cell
                .value()
                .attr("colspan")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);
            let rowspan = cell
                .value()
                .attr("rowspan")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);

            // セルの内容を取得
            let content = cell.text().collect::<String>().trim().to_string();

            // グリッドにセルを配置
            for r in 0..rowspan {
                let target_row = row_idx + r;

                // 行を拡張
                while grid.len() <= target_row {
                    grid.push(Vec::new());
                }

                for c in 0..colspan {
                    let target_col = col_idx + c;

                    // 列を拡張
                    while grid[target_row].len() <= target_col {
                        grid[target_row].push(None);
                    }

                    // 最初のセルには内容を、それ以外は空文字列を入れる
                    if r == 0 && c == 0 {
                        grid[target_row][target_col] = Some(content.clone());
                    } else {
                        grid[target_row][target_col] = Some(String::new());
                    }

                    max_cols = max_cols.max(target_col + 1);
                }
            }

            col_idx += colspan;
        }
    }

    // Option<String>からStringに変換し、すべての行を同じ列数に揃える
    grid.into_iter()
        .map(|row| {
            let mut normalized_row: Vec<String> = row
                .into_iter()
                .map(|cell| cell.unwrap_or_default())
                .collect();

            // 行の長さを最大列数に揃える
            while normalized_row.len() < max_cols {
                normalized_row.push(String::new());
            }

            normalized_row
        })
        .collect()
}

fn parse_tables_header_and_values(
    tables: &Vec<String>,
) -> Result<Vec<TableWithHeaderAndValues>, String> {
    let mut results = Vec::new();

    for table_html in tables {
        let html = Html::parse_fragment(table_html);
        let row_selector = Selector::parse("tr").unwrap();

        let rows: Vec<_> = html.select(&row_selector).collect();

        if rows.is_empty() {
            continue; // テーブルに行がない場合はスキップ
        }

        // rowspan/colspanを考慮してグリッドを正規化
        let normalized_grid = normalize_table_to_grid(&rows);

        if normalized_grid.is_empty() {
            continue;
        }

        // 最初の行をヘッダーとして扱う
        let headers = normalized_grid[0].clone();

        // 2行目以降をデータ行として扱う
        let data_rows: Vec<Vec<String>> = normalized_grid.into_iter().skip(1).collect();

        results.push(TableWithHeaderAndValues {
            headers,
            rows: data_rows,
        });
    }

    if results.is_empty() {
        return Err("No valid tables found".to_string());
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use super::*;

    #[test]
    fn test_get_tables_from_html_string() {
        // 取得結果を見れるだけのテスト
        let file_path =
            "/home/shotarokuramata/bort-scraping/src-tauri/bort-html/20250802/biyori.html";
        let input = fs::read_to_string(file_path);
        let s = input.unwrap();

        let r = get_tables_from_html_string(&s);
        for (i, result) in r.unwrap().iter().enumerate() {
            println!("table{}: {}", i, result);
            println!("\n -------------");
        }
    }

    #[test]
    fn test_parse_tables_header_and_values() {
        let file_path =
            "/home/shotarokuramata/bort-scraping/src-tauri/bort-html/20250802/biyori.html";
        let input = fs::read_to_string(file_path);
        let s = input.unwrap();

        let r = get_tables_from_html_string(&s).unwrap();
        let h = parse_tables_header_and_values(&r).unwrap();

        println!("Found {} tables", h.len());
        for (i, table) in h.iter().enumerate() {
            println!("\n=== Table {} ===", i);
            println!("Headers: {:?}", table.headers);
            println!("Data rows: {}", table.rows.len());
            // 最初の3行だけ表示
            for (row_idx, row) in table.rows.iter().take(3).enumerate() {
                println!("  Row {}: {:?}", row_idx, row);
            }
            println!("-------------");
        }
    }

    #[test]
    fn test_parse_table_data_valid() {
        let input = r#"
            <table class="table_fixed">
                <tr><th>列1</th><th>列2</th></tr>
                <tr><td>値1-1</td><td>値1-2</td></tr>
                <tr><td>値2-1</td><td>値2-2</td></tr>
            </table>
        "#;
        let result = parse_table_data(input).unwrap();

        assert!(result.line_count > 0);
        assert_eq!(result.data.len(), 1); // 1つのテーブル

        // JSON文字列をパースして検証
        let table: TableWithHeaderAndValues = serde_json::from_str(&result.data[0]).unwrap();
        assert_eq!(table.headers, vec!["列1", "列2"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["値1-1", "値1-2"]);
        assert_eq!(table.rows[1], vec!["値2-1", "値2-2"]);
    }

    #[test]
    fn test_parse_table_data_empty() {
        let input = "";
        let result = parse_table_data(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "入力データが空です");
    }

    #[test]
    fn test_parse_table_data_single_table() {
        let input = r#"
            <table class="table_fixed">
                <tr><th>ヘッダー</th></tr>
                <tr><td>データ</td></tr>
            </table>
        "#;
        let result = parse_table_data(input).unwrap();

        assert_eq!(result.data.len(), 1); // 1つのテーブル

        let table: TableWithHeaderAndValues = serde_json::from_str(&result.data[0]).unwrap();
        assert_eq!(table.headers, vec!["ヘッダー"]);
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0], vec!["データ"]);
    }

    #[test]
    fn test_normalize_table_with_colspan() {
        let input = r#"
            <table class="table_fixed">
                <tr>
                    <th colspan="3">タイトル</th>
                </tr>
                <tr>
                    <td>A</td>
                    <td>B</td>
                    <td>C</td>
                </tr>
            </table>
        "#;
        let result = parse_table_data(input).unwrap();
        let table: TableWithHeaderAndValues = serde_json::from_str(&result.data[0]).unwrap();

        // colspanで3列に展開されるので、ヘッダーも3列になる
        assert_eq!(table.headers.len(), 3);
        assert_eq!(table.headers[0], "タイトル");
        assert_eq!(table.headers[1], "");
        assert_eq!(table.headers[2], "");

        // データ行も3列
        assert_eq!(table.rows[0].len(), 3);
        assert_eq!(table.rows[0], vec!["A", "B", "C"]);
    }

    #[test]
    fn test_normalize_table_with_rowspan() {
        let input = r#"
            <table class="table_fixed">
                <tr>
                    <th>列1</th>
                    <th>列2</th>
                </tr>
                <tr>
                    <td rowspan="2">A</td>
                    <td>B1</td>
                </tr>
                <tr>
                    <td>B2</td>
                </tr>
            </table>
        "#;
        let result = parse_table_data(input).unwrap();
        let table: TableWithHeaderAndValues = serde_json::from_str(&result.data[0]).unwrap();

        assert_eq!(table.headers, vec!["列1", "列2"]);

        // rowspanで2行にまたがる
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["A", "B1"]);
        assert_eq!(table.rows[1], vec!["", "B2"]); // rowspanの2行目は空文字列
    }

    #[test]
    fn test_normalize_table_with_complex_spans() {
        let input = r#"
            <table class="table_fixed">
                <tr>
                    <th rowspan="2">枠</th>
                    <th colspan="2">データ</th>
                </tr>
                <tr>
                    <th>列A</th>
                    <th>列B</th>
                </tr>
                <tr>
                    <td>1</td>
                    <td>X</td>
                    <td>Y</td>
                </tr>
            </table>
        "#;
        let result = parse_table_data(input).unwrap();
        let table: TableWithHeaderAndValues = serde_json::from_str(&result.data[0]).unwrap();

        // すべての行が3列に正規化される
        assert_eq!(table.headers.len(), 3);
        assert_eq!(table.headers[0], "枠");
        assert_eq!(table.headers[1], "データ");
        assert_eq!(table.headers[2], "");

        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0].len(), 3);
        assert_eq!(table.rows[0], vec!["", "列A", "列B"]);
        assert_eq!(table.rows[1], vec!["1", "X", "Y"]);
    }
}

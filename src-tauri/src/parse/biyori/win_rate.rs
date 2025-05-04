// use scraper::{Html, Selector};

// pub fn get_win_rate_info(content: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
//     // HTMLをパース
//     let document = Html::parse_document(content);

//     // 必要なセレクタを定義
//     let race_basic_selector = Selector::parse("#raceBasic").unwrap();
//     let table_selector = Selector::parse("table.table_fixed").unwrap();
//     let row_selector = Selector::parse("tr").unwrap();
//     let cell_selector = Selector::parse("td").unwrap();

//     // raceBasic要素を取得
//     let race_basic = document
//         .select(&race_basic_selector)
//         .next()
//         .ok_or("raceBasic not found")?;
//     let table = race_basic
//         .select(&table_selector)
//         .next()
//         .ok_or("Table not found")?;

//     // 勝率を探す
//     let search_text = "勝率";
//     let mut found_rows = Vec::new();
//     let mut get_next_row = false;

//     for row in table.select(&row_selector) {
//         if get_next_row {
//             found_rows.push(row);
//             get_next_row = false;
//             continue;
//         }
//         for cell in row.select(&cell_selector) {
//             if cell.text().any(|text| text.contains(search_text)) {
//                 get_next_row = true;
//                 break;
//             }
//         }
//     }

//     // 見つけた行から値を抽出
//     let mut row_values = Vec::new();
//     for row in found_rows {
//         for cell in row.select(&cell_selector) {
//             let text = cell.text().collect::<Vec<_>>().join("").trim().to_string();
//             if let Ok(value) = text.parse::<f64>() {
//                 row_values.push(value);
//             }
//         }
//     }

//     Ok(row_values)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_get_win_rate_info() {
//         // テスト用のHTMLファイルのパス
//         let file_path = "./bort-html/20231001/biyori.html";

//         // HTMLファイルを読み込む
//         let content = fs::read_to_string(file_path).expect("Failed to read the test HTML file");

//         // 関数を実行
//         let result = get_win_rate_info(&content);

//         // 結果を確認
//         assert!(result.is_ok(), "Function returned an error: {:?}", result);

//         let win_rates = result.unwrap();

//         println!("Win rates: {:?}", win_rates);

//         // assert_eq!(
//         //     win_rates[0], 6.5,
//         //     "First win rate does not match expected value"
//         // );
//         // assert_eq!(
//         //     win_rates[1], 5.8,
//         //     "Second win rate does not match expected value"
//         // );
//     }
// }

use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug)]
pub struct RaceData {
    pub escape_last_year: f64,
    pub escape_last_half_year: f64,
    pub allow_escape_last_year: f64,
    pub allow_escape_last_half_year: f64,
    pub pierce_last_year: f64,
    pub overtake_last_year: f64,
    pub first_place_in_last_ten_race: usize,
}

impl RaceData {
    pub fn new() -> Self {
        RaceData {
            escape_last_year: 0.0,
            escape_last_half_year: 0.0,
            allow_escape_last_year: 0.0,
            allow_escape_last_half_year: 0.0,
            pierce_last_year: 0.0,
            overtake_last_year: 0.0,
            first_place_in_last_ten_race: 0,
        }
    }
}

impl fmt::Display for RaceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Escape Last Year: {:.2}%, Escape Last Half Year: {:.2}%, Allow Escape Last Year: {:.2}%, Allow Escape Last Half Year: {:.2}%, Pierce Last Year: {:.2}%, Overtake Last Year: {:.2}%, First Place in Last Ten Races: {}",
            self.escape_last_year * 100.0,
            self.escape_last_half_year * 100.0,
            self.allow_escape_last_year * 100.0,
            self.allow_escape_last_half_year * 100.0,
            self.pierce_last_year * 100.0,
            self.overtake_last_year * 100.0,
            self.first_place_in_last_ten_race
        )
    }
}

pub fn get_escaped_flame_info(content: &str) -> Result<RaceData, Box<dyn std::error::Error>> {
    let document = Html::parse_document(content);
    let race_basic_selector = Selector::parse("#raceBasic").unwrap();
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let race_basic = document
        .select(&race_basic_selector)
        .next()
        .ok_or("raceBasic not found")?;
    let table = race_basic
        .select(&table_selector)
        .next()
        .ok_or("Table not found")?;

    let mut race_data = RaceData::new();

    // 1年と半年の逃げ率と逃がし率
    let search_text = "逃げ";
    let mut found_rows = Vec::new();
    let mut get_next_row = false;

    for row in table.select(&row_selector) {
        if get_next_row {
            found_rows.push(row);
            get_next_row = false;
            continue;
        }
        for cell in row.select(&cell_selector) {
            if cell.text().any(|text| text.contains(search_text)) {
                get_next_row = true;
            }
        }
    }

    // 見つけた行から値を抽出
    let mut extracted_values = Vec::new();
    for row in found_rows {
        let row_values: Vec<_> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>().trim().to_string())
            .collect();
        extracted_values.push(row_values);
    }

    race_data.escape_last_year = from_percent_string_to_float(&extracted_values[0][0])?;
    race_data.escape_last_half_year = from_percent_string_to_float(&extracted_values[0][1])?;
    race_data.allow_escape_last_year = from_percent_string_to_float(&extracted_values[1][0])?;
    race_data.allow_escape_last_half_year = from_percent_string_to_float(&extracted_values[1][1])?;

    // 刺され率
    let search_text = "差され";
    let mut found_rows = Vec::new();
    let mut get_next_row = false;

    for row in table.select(&row_selector) {
        if get_next_row {
            found_rows.push(row);
            get_next_row = false;
            continue;
        }
        for cell in row.select(&cell_selector) {
            if cell.text().any(|text| text.contains(search_text)) {
                get_next_row = true;
            }
        }
    }

    // 見つけた行から値を抽出
    let mut extracted_values = Vec::new();
    for row in found_rows {
        let row_values: Vec<_> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>().trim().to_string())
            .collect();
        extracted_values.push(row_values);
    }

    race_data.pierce_last_year = from_percent_string_to_float(&extracted_values[1][0])?;

    // 捲られ率
    let search_text = "捲られ";
    let mut found_rows = Vec::new();
    let mut get_next_row = false;

    for row in table.select(&row_selector) {
        if get_next_row {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            if cells.is_empty() || cells[0].text().collect::<String>().trim().is_empty() {
                continue;
            }
            found_rows.push(row);
            get_next_row = false;
            continue;
        }
        for cell in row.select(&cell_selector) {
            if cell.text().any(|text| text.contains(search_text)) {
                get_next_row = true;
            }
        }
    }

    // 見つけた行から値を抽出
    let mut extracted_values = Vec::new();
    for row in found_rows {
        let row_values: Vec<_> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>().trim().to_string())
            .collect();
        extracted_values.push(row_values);
    }

    race_data.overtake_last_year = from_percent_string_to_float(&extracted_values[2][0])?;

    // 直近10レースで1着の回数
    let tables: Vec<_> = race_basic.select(&table_selector).collect();
    if tables.len() >= 6 {
        let table = &tables[5];
        let rows: Vec<_> = table.select(&row_selector).collect();
        if rows.len() > 5 {
            let row = &rows[5];
            let row_values: Vec<_> = row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            race_data.first_place_in_last_ten_race =
                row_values.iter().filter(|&v| v == "1").count();
        }
    }

    Ok(race_data)
}

fn from_percent_string_to_float(percent: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let value = percent.trim_end_matches('%').parse::<f64>()?;
    Ok(value / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_escaped_flame_info() {
        // テスト用のHTMLファイルのパス
        let file_path = "./bort-html/20231001/biyori.html";

        // HTMLファイルを読み込む
        let content = fs::read_to_string(file_path).expect("Failed to read the test HTML file");

        // 関数を実行
        let result = get_escaped_flame_info(&content);

        // エラー内容を確認
        if let Err(ref e) = result {
            eprintln!("Error occurred: {:?}", e); // エラーを標準エラー出力に表示
        }

        // 結果をデバッグ出力
        dbg!(&result);
    }
}

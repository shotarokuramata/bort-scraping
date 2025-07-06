use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug, serde::Serialize)]
pub struct RaceData {
    pub escape_last_year: f64,
    pub escape_last_half_year: f64,
    pub allow_escape_last_year: f64,
    pub allow_escape_last_half_year: f64,
    pub pierce_last_year: f64,
    pub pierce_last_half_year: f64,
    pub overtake_last_year: f64,
    pub overtake_last_half_year: f64,
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
            pierce_last_half_year: 0.0,
            overtake_last_year: 0.0,
            overtake_last_half_year: 0.0,
            first_place_in_last_ten_race: 0,
        }
    }
}

impl fmt::Display for RaceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Escape Last Year: {:.2}%, Escape Last Half Year: {:.2}%, Allow Escape Last Year: {:.2}%, Allow Escape Last Half Year: {:.2}%, Pierce Last Year: {:.2}%, Pierce Last Half Year: {:.2}%, Overtake Last Year: {:.2}%, Overtake Last Half Year: {:.2}%, First Place in Last Ten Races: {}",
            self.escape_last_year * 100.0,
            self.escape_last_half_year * 100.0,
            self.allow_escape_last_year * 100.0,
            self.allow_escape_last_half_year * 100.0,
            self.pierce_last_year * 100.0,
            self.pierce_last_half_year * 100.0,
            self.overtake_last_year * 100.0,
            self.overtake_last_half_year * 100.0,
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

    // 正しいデータマッピング - 実際のテーブル順序に合わせて修正
    // extracted_values[0] = 逃げ率のデータ (1年間, 半年間)
    // extracted_values[1] = 逃がし率のデータ (1年間, 半年間)
    race_data.escape_last_year = from_percent_string_to_float(&extracted_values[1][0])?;      // 1年間逃げ率
    race_data.escape_last_half_year = from_percent_string_to_float(&extracted_values[0][0])?; // 半年間逃げ率
    race_data.allow_escape_last_year = from_percent_string_to_float(&extracted_values[1][1])?; // 1年間逃がし率
    race_data.allow_escape_last_half_year = from_percent_string_to_float(&extracted_values[0][1])?; // 半年間逃がし率

    // 刺され率（1号艇のみ）
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

    // 見つけた行から値を抽出（1号艇のデータのみ）
    let mut extracted_values = Vec::new();
    for row in found_rows {
        let row_values: Vec<_> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>().trim().to_string())
            .collect();
        extracted_values.push(row_values);
    }

    // データ行0は半年間、データ行1は1年間のデータ
    // 1号艇のデータは各行の最初の列（インデックス0）
    race_data.pierce_last_half_year = from_percent_string_to_float(&extracted_values[0][0])?; // 半年間差され率
    race_data.pierce_last_year = from_percent_string_to_float(&extracted_values[1][0])?; // 1年間差され率

    // 捲られ率（1号艇のみ）- 差され率と同じシンプルなアプローチ + 空行スキップ
    let search_text = "捲られ";
    let mut found_rows = Vec::new();
    let mut get_next_row = false;

    for row in table.select(&row_selector) {
        if get_next_row {
            // 空行をスキップして、実際にデータが含まれている行のみを取得
            let row_values: Vec<String> = row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            
            // 空行でなく、かつパーセンテージデータが含まれている行のみを対象とする
            if !row_values.is_empty() && row_values.iter().any(|val| val.contains('%')) {
                found_rows.push(row);
                get_next_row = false;
            }
            continue;
        }
        for cell in row.select(&cell_selector) {
            if cell.text().any(|text| text.contains(search_text)) {
                get_next_row = true;
            }
        }
    }

    // 見つけた行から値を抽出（1号艇のデータのみ）
    let mut extracted_values = Vec::new();
    for row in found_rows {
        let row_values: Vec<_> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>().trim().to_string())
            .collect();
        extracted_values.push(row_values);
    }

    // 捲られ率のデータ構造: データ行0は半年間、データ行2は1年間（行1,3は別項目）
    // 1号艇のデータは各行の最初の列（インデックス0）
    if extracted_values.len() >= 3 {
        race_data.overtake_last_half_year = from_percent_string_to_float(&extracted_values[0][0])?; // 半年間捲られ率
        race_data.overtake_last_year = from_percent_string_to_float(&extracted_values[2][0])?; // 1年間捲られ率
    }

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

    #[test]
    fn test_get_escaped_flame_info_20250705() {
        // 20250705のテストデータで期待値を検証
        let file_path = "./bort-html/20250705/biyori.html";

        // HTMLファイルを読み込む
        let content = fs::read_to_string(file_path).expect("Failed to read the test HTML file");

        // 関数を実行
        let result = get_escaped_flame_info(&content);

        match result {
            Ok(race_data) => {
                println!("=== 20250705 レースデータ ===");
                println!("逃げ率（1年間）: {:.1}%", race_data.escape_last_year * 100.0);
                println!("逃げ率（半年間）: {:.1}%", race_data.escape_last_half_year * 100.0);
                println!("逃がし率（1年間）: {:.1}%", race_data.allow_escape_last_year * 100.0);
                println!("逃がし率（半年間）: {:.1}%", race_data.allow_escape_last_half_year * 100.0);
                println!("差され率（1年間）: {:.1}%", race_data.pierce_last_year * 100.0);
                println!("差され率（半年間）: {:.1}%", race_data.pierce_last_half_year * 100.0);
                println!("捲られ率（1年間）: {:.1}%", race_data.overtake_last_year * 100.0);
                println!("捲られ率（半年間）: {:.1}%", race_data.overtake_last_half_year * 100.0);
                println!("直近10レース1着回数: {}", race_data.first_place_in_last_ten_race);

                // 期待値との比較（20250705データ基準）
                println!("\n=== 期待値との比較 ===");
                println!("逃げ率（1年間）期待値: 31.0%, 実際値: {:.1}%", race_data.escape_last_year * 100.0);
                println!("逃げ率（半年間）期待値: 18.8%, 実際値: {:.1}%", race_data.escape_last_half_year * 100.0);
                println!("逃がし率（1年間）期待値: 62.2%, 実際値: {:.1}%", race_data.allow_escape_last_year * 100.0);
                println!("逃がし率（半年間）期待値: 64.0%, 実際値: {:.1}%", race_data.allow_escape_last_half_year * 100.0);
                println!("差され率（1年間）期待値: 17.2%, 実際値: {:.1}%", race_data.pierce_last_year * 100.0);
                println!("差され率（半年間）期待値: 25.0%, 実際値: {:.1}%", race_data.pierce_last_half_year * 100.0);
                println!("捲られ率（1年間）期待値: 27.6%, 実際値: {:.1}%", race_data.overtake_last_year * 100.0);
                println!("捲られ率（半年間）期待値: 31.3%, 実際値: {:.1}%", race_data.overtake_last_half_year * 100.0);
                println!("直近10レース1着回数期待値: 1回, 実際値: {}回", race_data.first_place_in_last_ten_race);

                // 全項目のアサーション（許容誤差0.1%で比較）
                assert!((race_data.escape_last_year * 100.0 - 31.0).abs() < 0.1, 
                    "逃げ率（1年間）が期待値と異なります: 期待31.0%, 実際{:.1}%", race_data.escape_last_year * 100.0);
                assert!((race_data.escape_last_half_year * 100.0 - 18.8).abs() < 0.1, 
                    "逃げ率（半年間）が期待値と異なります: 期待18.8%, 実際{:.1}%", race_data.escape_last_half_year * 100.0);
                assert!((race_data.allow_escape_last_year * 100.0 - 62.2).abs() < 0.1, 
                    "逃がし率（1年間）が期待値と異なります: 期待62.2%, 実際{:.1}%", race_data.allow_escape_last_year * 100.0);
                assert!((race_data.allow_escape_last_half_year * 100.0 - 64.0).abs() < 0.1, 
                    "逃がし率（半年間）が期待値と異なります: 期待64.0%, 実際{:.1}%", race_data.allow_escape_last_half_year * 100.0);
                assert!((race_data.pierce_last_year * 100.0 - 17.2).abs() < 0.1, 
                    "差され率（1年間）が期待値と異なります: 期待17.2%, 実際{:.1}%", race_data.pierce_last_year * 100.0);
                assert!((race_data.pierce_last_half_year * 100.0 - 25.0).abs() < 0.1, 
                    "差され率（半年間）が期待値と異なります: 期待25.0%, 実際{:.1}%", race_data.pierce_last_half_year * 100.0);
                assert!((race_data.overtake_last_year * 100.0 - 27.6).abs() < 0.1, 
                    "捲られ率（1年間）が期待値と異なります: 期待27.6%, 実際{:.1}%", race_data.overtake_last_year * 100.0);
                assert!((race_data.overtake_last_half_year * 100.0 - 31.3).abs() < 0.1, 
                    "捲られ率（半年間）が期待値と異なります: 期待31.3%, 実際{:.1}%", race_data.overtake_last_half_year * 100.0);
                assert_eq!(race_data.first_place_in_last_ten_race, 1, 
                    "直近10レース1着回数が期待値と異なります: 期待1回, 実際{}回", race_data.first_place_in_last_ten_race);
            }
            Err(e) => {
                panic!("テストでエラーが発生しました: {:?}", e);
            }
        }
    }

    #[test]
    fn test_debug_table_structure() {
        // HTMLの構造を詳しく調べるためのテスト
        let file_path = "./bort-html/20250705/biyori.html";
        let content = fs::read_to_string(file_path).expect("Failed to read the test HTML file");
        
        use scraper::{Html, Selector};
        let document = Html::parse_document(&content);
        let race_basic_selector = Selector::parse("#raceBasic").unwrap();
        let table_selector = Selector::parse("table.table_fixed").unwrap();
        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        let race_basic = document.select(&race_basic_selector).next().unwrap();
        let table = race_basic.select(&table_selector).next().unwrap();

        println!("\n=== HTMLテーブル構造の解析 ===");

        // 差され率の行を探す
        let search_text = "差され";
        let mut found_rows = Vec::new();
        let mut get_next_row = false;

        for (row_idx, row) in table.select(&row_selector).enumerate() {
            let row_text: String = row.text().collect();
            println!("行 {}: {}", row_idx, row_text.trim());

            if get_next_row {
                found_rows.push(row);
                println!("  -> データ行として取得");
                get_next_row = false;
                continue;
            }
            for cell in row.select(&cell_selector) {
                if cell.text().any(|text| text.contains(search_text)) {
                    println!("  -> '{}' を含む行を発見", search_text);
                    get_next_row = true;
                }
            }
        }

        println!("\n=== 差され率データ行の内容 ===");
        for (idx, row) in found_rows.iter().enumerate() {
            let row_values: Vec<String> = row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            println!("データ行 {}: {:?}", idx, row_values);
        }

        // 捲られ率の構造も確認
        println!("\n=== 捲られ率の構造確認 ===");
        let search_text = "捲られ";
        let mut found_rows = Vec::new();
        let mut get_next_row = false;

        for (row_idx, row) in table.select(&row_selector).enumerate() {
            let row_text: String = row.text().collect();
            
            if get_next_row {
                found_rows.push(row);
                println!("捲られ率データ行 {}: {}", found_rows.len() - 1, row_text.trim());
                get_next_row = false;
                continue;
            }
            for cell in row.select(&cell_selector) {
                if cell.text().any(|text| text.contains(search_text)) {
                    println!("捲られ率ヘッダー行 {}: {}", row_idx, row_text.trim());
                    get_next_row = true;
                }
            }
        }

        println!("\n=== 捲られ率データ行の内容 ===");
        for (idx, row) in found_rows.iter().enumerate() {
            let row_values: Vec<String> = row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            println!("捲られ率データ行 {}: {:?}", idx, row_values);
        }
    }
}

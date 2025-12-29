use scraper::{Html, Selector};

// Import data models from centralized models module
pub use crate::models::race::*;

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

    // 選手基本情報を抽出
    race_data.player_basic_info = extract_player_basic_info(&document)?;

    // 詳細成績データを抽出
    race_data.detailed_performance = extract_detailed_performance_data(&document)?;

    // ST関連データを抽出
    race_data.st_data = extract_st_related_data(&document)?;

    // 決まり手データを抽出
    race_data.winning_hand = extract_winning_hand_data(&document)?;

    Ok(race_data)
}

fn extract_player_basic_info(document: &Html) -> Result<PlayerBasicInfo, Box<dyn std::error::Error>> {
    let race_member_names_selector = Selector::parse("#raceMemerNames").unwrap();
    let race_member_names2_selector = Selector::parse("#raceMemerNames2").unwrap();
    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let course1_selector = Selector::parse("td.course1").unwrap();

    let mut player_info = PlayerBasicInfo::new();

    // 1号艇の基本情報を抽出
    // #raceMemerNames から 登録番号、名前、級別を取得
    if let Some(race_member_names) = document.select(&race_member_names_selector).next() {
        if let Some(table) = race_member_names.select(&table_selector).next() {
            let rows: Vec<_> = table.select(&row_selector).collect();
            
            // 2行目：登録番号
            if let Some(row) = rows.get(1) {
                if let Some(cell) = row.select(&course1_selector).next() {
                    player_info.registration_number = cell.text().collect::<String>().trim().to_string();
                }
            }
            
            // 3行目：選手名
            if let Some(row) = rows.get(2) {
                if let Some(cell) = row.select(&course1_selector).next() {
                    // 性別の判定（女性選手の場合はjoshi.pngがある）
                    let has_female_image = cell.select(&Selector::parse("img").unwrap())
                        .any(|img| img.value().attr("src").is_some_and(|src| src.contains("joshi.png")));
                    
                    player_info.gender = if has_female_image { "女性".to_string() } else { "男性".to_string() };
                    
                    // data-player_name属性から名前を取得
                    if let Some(player_name) = cell.value().attr("data-player_name") {
                        player_info.name = player_name.to_string();
                    } else {
                        // fallback: テキストコンテンツから名前を取得
                        player_info.name = cell.text().collect::<String>().trim().to_string();
                    }
                }
            }
            
            // 4行目：級別
            if let Some(row) = rows.get(3) {
                if let Some(cell) = row.select(&course1_selector).next() {
                    player_info.class_level = cell.text().collect::<String>().trim().to_string();
                }
            }
        }
    }

    // #raceMemerNames2 から 期別、支部を取得
    if let Some(race_member_names2) = document.select(&race_member_names2_selector).next() {
        if let Some(table) = race_member_names2.select(&table_selector).next() {
            let rows: Vec<_> = table.select(&row_selector).collect();
            
            // 1行目：期別と支部
            if let Some(row) = rows.first() {
                if let Some(cell) = row.select(&course1_selector).next() {
                    // HTMLの構造: "65期<br><span>群馬</span>" または "65期<br>群馬"
                    let cell_html = cell.html();
                    
                    // 期別を抽出（<br>タグの前の部分）
                    if let Some(br_pos) = cell_html.find("<br>") {
                        let period_part = &cell_html[..br_pos];
                        // HTMLタグを除去して期別を抽出
                        let period_text = Html::parse_fragment(period_part);
                        player_info.period = period_text.root_element().text().collect::<String>().trim().to_string();
                    }
                    
                    // 支部を抽出（<span>タグ内またはテキストから）
                    let span_selector = Selector::parse("span").unwrap();
                    if let Some(span_element) = cell.select(&span_selector).next() {
                        // <span>タグ内の支部を取得
                        player_info.support_group = span_element.text().collect::<String>().trim().to_string();
                    } else {
                        // <span>タグがない場合は、<br>タグ以降のテキストを取得
                        let all_text = cell.text().collect::<String>();
                        let parts: Vec<&str> = all_text.split_whitespace().collect();
                        if parts.len() >= 2 {
                            player_info.support_group = parts[1].to_string();
                        }
                    }
                }
            }
        }
    }

    Ok(player_info)
}

fn extract_detailed_performance_data(document: &Html) -> Result<DetailedPerformanceData, Box<dyn std::error::Error>> {
    let race_basic_selector = Selector::parse("#raceBasic").unwrap();
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let row_selector = Selector::parse("tr").unwrap();

    let race_basic = document
        .select(&race_basic_selector)
        .next()
        .ok_or("raceBasic not found")?;
    let table = race_basic
        .select(&table_selector)
        .next()
        .ok_or("Table not found")?;

    let mut detailed_performance = DetailedPerformanceData::new();

    // テーブルの全行を取得
    let rows: Vec<_> = table.select(&row_selector).collect();

    // 1着率、枠別勝率のデータを抽出
    let performance_metrics = [
        ("1着率", &mut detailed_performance.first_place_rate),
    ];

    for (metric_name, performance_data) in performance_metrics {
        extract_performance_metric_data(&rows, metric_name, performance_data)?;
    }

    // 枠別勝率データを個別に処理
    extract_lane_win_rate_data(&rows, &mut detailed_performance.lane_win_rate)?;

    Ok(detailed_performance)
}

fn extract_performance_metric_data(
    rows: &[scraper::ElementRef],
    metric_name: &str,
    performance_data: &mut PerformanceData,
) -> Result<(), Box<dyn std::error::Error>> {
    let cell_selector = Selector::parse("td").unwrap();

    // ヘッダー行を探す
    let mut metric_start_index = None;
    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        for cell in cells {
            let text = cell.text().collect::<String>();
            if text.trim() == metric_name {
                metric_start_index = Some(i + 1); // 次の行からデータが開始
                break;
            }
        }
        if metric_start_index.is_some() {
            break;
        }
    }

    if let Some(start_index) = metric_start_index {
        // 時期別データを抽出
        let time_periods = [
            ("今期", &mut performance_data.this_period),
            ("直近6ヶ月", &mut performance_data.last_6_months),
            ("直近3ヶ月", &mut performance_data.last_3_months),
            ("直近1ヶ月", &mut performance_data.last_1_month),
            ("当地", &mut performance_data.local_venue),
            ("一般戦", &mut performance_data.general_races),
            ("SG/G1", &mut performance_data.sg_g1),
        ];

        for (period_name, data_ref) in time_periods {
            if let Some(value) = find_period_data_for_boat1(rows, start_index, period_name)? {
                *data_ref = Some(value);
            }
        }
    }

    Ok(())
}

fn extract_lane_win_rate_data(
    rows: &[scraper::ElementRef],
    lane_win_rate: &mut LaneWinRateData,
) -> Result<(), Box<dyn std::error::Error>> {
    let cell_selector = Selector::parse("td").unwrap();

    // 枠別勝率のヘッダー行を探す
    let mut metric_start_index = None;
    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        for cell in cells {
            let text = cell.text().collect::<String>();
            if text.trim() == "枠別勝率" {
                metric_start_index = Some(i + 1);
                break;
            }
        }
        if metric_start_index.is_some() {
            break;
        }
    }

    if let Some(start_index) = metric_start_index {
        // 枠別勝率の時期別データを抽出
        let time_periods = [
            ("直近1年", &mut lane_win_rate.last_1_year),
            ("直近6ヵ月", &mut lane_win_rate.last_6_months),
        ];

        for (period_name, data_ref) in time_periods {
            if let Some(value) = find_period_data_for_boat1(rows, start_index, period_name)? {
                *data_ref = Some(value);
            }
        }
    }

    Ok(())
}

fn find_period_data_for_boat1(
    rows: &[scraper::ElementRef],
    start_index: usize,
    period_name: &str,
) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let cell_selector = Selector::parse("td").unwrap();

    // 指定された時期のデータ行を探す
    for i in start_index..rows.len().min(start_index + 10) {
        if let Some(row) = rows.get(i) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            
            // 最初のセルが時期名かチェック
            if let Some(first_cell) = cells.first() {
                let first_cell_text = first_cell.text().collect::<String>();
                if first_cell_text.trim() == period_name {
                    // 1号艇のデータは2番目のセル（インデックス1）
                    if let Some(boat1_cell) = cells.get(1) {
                        let boat1_text = boat1_cell.text().collect::<String>();
                        return parse_performance_value(&boat1_text);
                    }
                }
            }
        }
    }

    Ok(None)
}

fn parse_performance_value(text: &str) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let cleaned_text = text.trim();
    
    // "-" の場合はデータなし
    if cleaned_text == "-" || cleaned_text.is_empty() {
        return Ok(None);
    }

    // パーセンテージの部分を抽出
    if let Some(percent_pos) = cleaned_text.find('%') {
        let percent_str = &cleaned_text[..percent_pos];
        match percent_str.parse::<f64>() {
            Ok(value) => Ok(Some(value / 100.0)),
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn extract_st_related_data(document: &Html) -> Result<STRelatedData, Box<dyn std::error::Error>> {
    let race_basic_selector = Selector::parse("#raceBasic").unwrap();
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let row_selector = Selector::parse("tr").unwrap();

    let race_basic = document
        .select(&race_basic_selector)
        .next()
        .ok_or("raceBasic not found")?;
    let table = race_basic
        .select(&table_selector)
        .next()
        .ok_or("Table not found")?;

    let mut st_data = STRelatedData::new();

    // テーブルの全行を取得
    let rows: Vec<_> = table.select(&row_selector).collect();

    // 平均STとST順位のデータを抽出
    let st_metrics = [
        ("平均ST", &mut st_data.average_st),
        ("ST順位", &mut st_data.st_ranking),
    ];

    for (metric_name, st_metric_data) in st_metrics {
        extract_st_metric_data(&rows, metric_name, st_metric_data)?;
    }

    // ST考察データを個別に処理
    extract_st_analysis_data(&rows, &mut st_data.st_analysis)?;

    Ok(st_data)
}

fn extract_st_metric_data(
    rows: &[scraper::ElementRef],
    metric_name: &str,
    st_data: &mut STData,
) -> Result<(), Box<dyn std::error::Error>> {
    let cell_selector = Selector::parse("td").unwrap();

    // ヘッダー行を探す
    let mut metric_start_index = None;
    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        for cell in cells {
            let text = cell.text().collect::<String>();
            if text.trim() == metric_name {
                metric_start_index = Some(i + 1); // 次の行からデータが開始
                break;
            }
        }
        if metric_start_index.is_some() {
            break;
        }
    }

    if let Some(start_index) = metric_start_index {
        // 時期別データを抽出
        let time_periods = [
            ("今期", &mut st_data.this_period),
            ("直近6ヶ月", &mut st_data.last_6_months),
            ("直近3ヶ月", &mut st_data.last_3_months),
            ("直近1ヶ月", &mut st_data.last_1_month),
            ("当地", &mut st_data.local_venue),
            ("一般戦", &mut st_data.general_races),
            ("SG/G1", &mut st_data.sg_g1),
            ("初日", &mut st_data.first_day),
            ("最終日", &mut st_data.final_day),
            ("ナイター", &mut st_data.night_races),
            ("F持", &mut st_data.flying_history),
        ];

        for (period_name, data_ref) in time_periods {
            if let Some(value) = find_st_period_data_for_boat1(rows, start_index, period_name)? {
                *data_ref = Some(value);
            }
        }
    }

    Ok(())
}

fn extract_st_analysis_data(
    rows: &[scraper::ElementRef],
    st_analysis: &mut STAnalysisData,
) -> Result<(), Box<dyn std::error::Error>> {
    let cell_selector = Selector::parse("td").unwrap();

    // ST考察のヘッダー行を探す
    let mut metric_start_index = None;
    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        for cell in cells {
            let text = cell.text().collect::<String>();
            if text.trim() == "ST考察" {
                metric_start_index = Some(i + 1);
                break;
            }
        }
        if metric_start_index.is_some() {
            break;
        }
    }

    if let Some(start_index) = metric_start_index {
        // ST考察の各項目データを抽出
        let analysis_items = [
            ("安定率", &mut st_analysis.stability_rate),
            ("抜出率", &mut st_analysis.break_out_rate),
            ("出遅率", &mut st_analysis.late_start_rate),
        ];

        for (item_name, data_ref) in analysis_items {
            if let Some(value) = find_st_period_data_for_boat1(rows, start_index, item_name)? {
                *data_ref = Some(value);
            }
        }
    }

    Ok(())
}

fn find_st_period_data_for_boat1(
    rows: &[scraper::ElementRef],
    start_index: usize,
    period_name: &str,
) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let cell_selector = Selector::parse("td").unwrap();

    // 指定された時期のデータ行を探す
    for i in start_index..rows.len().min(start_index + 15) {
        if let Some(row) = rows.get(i) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            
            // 最初のセルが時期名かチェック
            if let Some(first_cell) = cells.first() {
                let first_cell_text = first_cell.text().collect::<String>();
                if first_cell_text.trim() == period_name {
                    // 1号艇のデータは2番目のセル（インデックス1）
                    if let Some(boat1_cell) = cells.get(1) {
                        let boat1_text = boat1_cell.text().collect::<String>();
                        return parse_st_value(&boat1_text);
                    }
                }
            }
        }
    }

    Ok(None)
}

fn parse_st_value(text: &str) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let cleaned_text = text.trim();
    
    // "-" の場合はデータなし
    if cleaned_text == "-" || cleaned_text.is_empty() {
        return Ok(None);
    }

    // パーセンテージ値の場合（ST考察データ）
    if let Some(percent_pos) = cleaned_text.find('%') {
        let percent_str = &cleaned_text[..percent_pos];
        match percent_str.parse::<f64>() {
            Ok(value) => Ok(Some(value / 100.0)),
            Err(_) => Ok(None),
        }
    } else {
        // 数値の場合（平均STやST順位）
        match cleaned_text.parse::<f64>() {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

fn extract_winning_hand_data(document: &Html) -> Result<WinningHandData, Box<dyn std::error::Error>> {
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

    let mut winning_hand = WinningHandData::new();

    // テーブルの全行を取得
    let rows: Vec<_> = table.select(&row_selector).collect();

    // 決まり手セクションを探す
    let mut winning_hand_start_index = None;
    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        for cell in cells {
            let text = cell.text().collect::<String>();
            if text.trim() == "決まり手" {
                winning_hand_start_index = Some(i);
                break;
            }
        }
        if winning_hand_start_index.is_some() {
            break;
        }
    }

    if let Some(start_index) = winning_hand_start_index {
        // 決まり手データを構造化して抽出（直近6ヶ月）
        
        // 逃げ/逃しの組み合わせを探す (行63-64)
        for i in start_index..rows.len().min(start_index + 10) {
            if let Some(row) = rows.get(i) {
                let row_text = row.text().collect::<String>();
                if row_text.contains("逃げ") && row_text.contains("逃し") {
                    // 次の行に逃げ率と逃し率のデータがある
                    if let Some(data_row) = rows.get(i + 1) {
                        let data_cells: Vec<_> = data_row.select(&cell_selector).collect();
                        if data_cells.len() >= 2 {
                            // 1つ目のセルが逃げ率、2つ目のセルが逃し率
                            if let Some(escape_cell) = data_cells.first() {
                                let escape_text = escape_cell.text().collect::<String>();
                                if let Ok(Some(value)) = parse_winning_hand_value(&escape_text) {
                                    winning_hand.escape_rate_6months = Some(value);
                                }
                            }
                            if let Some(let_escape_cell) = data_cells.get(1) {
                                let let_escape_text = let_escape_cell.text().collect::<String>();
                                if let Ok(Some(value)) = parse_winning_hand_value(&let_escape_text) {
                                    winning_hand.let_escape_rate_6months = Some(value);
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }

        // 差され/差しの組み合わせを探す (行65-66)
        for i in start_index..rows.len().min(start_index + 10) {
            if let Some(row) = rows.get(i) {
                let row_text = row.text().collect::<String>();
                if row_text.contains("差され") && row_text.contains("差し") {
                    // 次の行に差され率と各号艇の差し率のデータがある
                    if let Some(data_row) = rows.get(i + 1) {
                        let data_cells: Vec<_> = data_row.select(&cell_selector).collect();
                        if data_cells.len() >= 2 {
                            // 1つ目のセルが差され率、2つ目のセルが2号艇の差し率
                            if let Some(pierced_cell) = data_cells.first() {
                                let pierced_text = pierced_cell.text().collect::<String>();
                                if let Ok(Some(value)) = parse_winning_hand_value(&pierced_text) {
                                    winning_hand.pierced_rate_6months = Some(value);
                                }
                            }
                            if let Some(pierce_cell) = data_cells.get(1) {
                                let pierce_text = pierce_cell.text().collect::<String>();
                                if let Ok(Some(value)) = parse_winning_hand_value(&pierce_text) {
                                    winning_hand.pierce_rate_6months = Some(value);
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }

        // 捲られ/捲りの組み合わせを探す (行67-69、空行をスキップ)
        for i in start_index..rows.len().min(start_index + 15) {
            if let Some(row) = rows.get(i) {
                let row_text = row.text().collect::<String>();
                if row_text.contains("捲られ") && row_text.contains("捲り") {
                    // 次の行が空行で、その次の行に捲られ率と捲り率のデータがある
                    if let Some(data_row) = rows.get(i + 2) { // 空行をスキップして2行後
                        let data_cells: Vec<_> = data_row.select(&cell_selector).collect();
                        if data_cells.len() >= 2 {
                            // 1つ目のセルが捲られ率、2つ目のセルが捲り率
                            if let Some(overtaken_cell) = data_cells.first() {
                                let overtaken_text = overtaken_cell.text().collect::<String>();
                                if let Ok(Some(value)) = parse_winning_hand_value(&overtaken_text) {
                                    winning_hand.overtake_rate_6months = Some(value);
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    Ok(winning_hand)
}


fn parse_winning_hand_value(text: &str) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let cleaned_text = text.trim();
    
    // "-" の場合はデータなし
    if cleaned_text == "-" || cleaned_text.is_empty() {
        return Ok(None);
    }

    // パーセンテージの部分を抽出
    if let Some(percent_pos) = cleaned_text.find('%') {
        let percent_str = &cleaned_text[..percent_pos];
        match percent_str.parse::<f64>() {
            Ok(value) => Ok(Some(value / 100.0)),
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
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
                println!("\n=== 選手基本情報 ===");
                println!("登録番号: {}", race_data.player_basic_info.registration_number);
                println!("選手名: {}", race_data.player_basic_info.name);
                println!("級別: {}", race_data.player_basic_info.class_level);
                println!("期別: {}", race_data.player_basic_info.period);
                println!("支部: {}", race_data.player_basic_info.support_group);
                println!("性別: {}", race_data.player_basic_info.gender);

                // 詳細成績データの表示
                println!("\n=== 詳細成績データ ===");
                
                println!("【1着率】");
                if let Some(v) = race_data.detailed_performance.first_place_rate.this_period { println!("  今期: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.first_place_rate.last_6_months { println!("  直近6ヶ月: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.first_place_rate.last_3_months { println!("  直近3ヶ月: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.first_place_rate.last_1_month { println!("  直近1ヶ月: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.first_place_rate.local_venue { println!("  当地: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.first_place_rate.general_races { println!("  一般戦: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.first_place_rate.sg_g1 { println!("  SG/G1: {:.1}%", v * 100.0); }

                println!("【枠別勝率】");
                if let Some(v) = race_data.detailed_performance.lane_win_rate.last_1_year { println!("  直近1年: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.detailed_performance.lane_win_rate.last_6_months { println!("  直近6ヵ月: {:.1}%", v * 100.0); }

                // ST関連データの表示
                println!("\n=== ST関連データ ===");
                
                println!("【平均ST】");
                if let Some(v) = race_data.st_data.average_st.this_period { println!("  今期: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.last_6_months { println!("  直近6ヶ月: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.last_3_months { println!("  直近3ヶ月: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.last_1_month { println!("  直近1ヶ月: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.local_venue { println!("  当地: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.general_races { println!("  一般戦: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.sg_g1 { println!("  SG/G1: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.first_day { println!("  初日: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.final_day { println!("  最終日: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.night_races { println!("  ナイター: {:.2}", v); }
                if let Some(v) = race_data.st_data.average_st.flying_history { println!("  F持: {:.2}", v); }

                println!("【ST順位】");
                if let Some(v) = race_data.st_data.st_ranking.this_period { println!("  今期: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.last_6_months { println!("  直近6ヶ月: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.last_3_months { println!("  直近3ヶ月: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.last_1_month { println!("  直近1ヶ月: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.local_venue { println!("  当地: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.general_races { println!("  一般戦: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.sg_g1 { println!("  SG/G1: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.first_day { println!("  初日: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.final_day { println!("  最終日: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.night_races { println!("  ナイター: {:.2}", v); }
                if let Some(v) = race_data.st_data.st_ranking.flying_history { println!("  F持: {:.2}", v); }

                println!("【ST考察】");
                if let Some(v) = race_data.st_data.st_analysis.stability_rate { println!("  安定率: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.st_data.st_analysis.break_out_rate { println!("  抜出率: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.st_data.st_analysis.late_start_rate { println!("  出遅率: {:.1}%", v * 100.0); }

                // 決まり手データの表示
                println!("\n=== 決まり手データ（直近6ヶ月） ===");
                if let Some(v) = race_data.winning_hand.escape_rate_6months { println!("逃げ率: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.winning_hand.let_escape_rate_6months { println!("逃し率: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.winning_hand.pierced_rate_6months { println!("差され率: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.winning_hand.pierce_rate_6months { println!("差し率: {:.1}%", v * 100.0); }
                if let Some(v) = race_data.winning_hand.overtake_rate_6months { println!("捲り率: {:.1}%", v * 100.0); }

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

                // 選手基本情報のアサーション（20250705データ基準：1号艇 高山秀雄選手）
                assert_eq!(race_data.player_basic_info.registration_number, "3448", 
                    "登録番号が期待値と異なります: 期待3448, 実際{}", race_data.player_basic_info.registration_number);
                assert_eq!(race_data.player_basic_info.name, "高山秀雄", 
                    "選手名が期待値と異なります: 期待高山秀雄, 実際{}", race_data.player_basic_info.name);
                assert_eq!(race_data.player_basic_info.class_level, "B1", 
                    "級別が期待値と異なります: 期待B1, 実際{}", race_data.player_basic_info.class_level);
                assert_eq!(race_data.player_basic_info.period, "65期", 
                    "期別が期待値と異なります: 期待65期, 実際{}", race_data.player_basic_info.period);
                assert_eq!(race_data.player_basic_info.support_group, "群馬", 
                    "支部が期待値と異なります: 期待群馬, 実際{}", race_data.player_basic_info.support_group);
                assert_eq!(race_data.player_basic_info.gender, "男性", 
                    "性別が期待値と異なります: 期待男性, 実際{}", race_data.player_basic_info.gender);

                // 決まり手データのアサーション（20250705データ基準）
                if let Some(escape_rate) = race_data.winning_hand.escape_rate_6months {
                    assert!((escape_rate * 100.0 - 18.8).abs() < 0.1, 
                        "決まり手・逃げ率（6ヶ月）が期待値と異なります: 期待18.8%, 実際{:.1}%", escape_rate * 100.0);
                }
                if let Some(let_escape_rate) = race_data.winning_hand.let_escape_rate_6months {
                    assert!((let_escape_rate * 100.0 - 64.0).abs() < 0.1, 
                        "決まり手・逃し率（6ヶ月）が期待値と異なります: 期待64.0%, 実際{:.1}%", let_escape_rate * 100.0);
                }
                if let Some(pierced_rate) = race_data.winning_hand.pierced_rate_6months {
                    assert!((pierced_rate * 100.0 - 25.0).abs() < 0.1, 
                        "決まり手・差され率（6ヶ月）が期待値と異なります: 期待25.0%, 実際{:.1}%", pierced_rate * 100.0);
                }
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

// オッズデータ解析関数
// 単勝・複勝オッズを解析する関数
pub fn parse_win_place_odds_from_html(html_content: &str) -> Result<OddsData, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html_content);
    let table_selector = Selector::parse("table.odds_table")?;
    let row_selector = Selector::parse("tr")?;
    let cell_selector = Selector::parse("td")?;
    
    let mut combinations = Vec::new();
    
    // オッズテーブルを探す
    if let Some(odds_table) = document.select(&table_selector).next() {
        println!("単勝・複勝オッズテーブルが見つかりました");
        
        let rows: Vec<_> = odds_table.select(&row_selector).collect();
        
        // ヘッダー行をスキップ（class="table_top_title"）
        for row in rows.iter() {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            
            // ヘッダー行はスキップ
            if cells.iter().any(|cell| cell.value().attr("class") == Some("table_top_title")) {
                continue;
            }
            
            // 各行の構造: [コース番号, 選手情報, 単勝オッズ, 複勝オッズ]
            if cells.len() >= 4 {
                // コース番号を取得（class="course1"などから）
                let course_cell = &cells[0];
                let course_text = course_cell.text().collect::<String>().trim().to_string();
                
                if let Ok(course_number) = course_text.parse::<u8>() {
                    // 単勝オッズ（3列目）
                    let win_odds_text = cells[2].text().collect::<String>().trim().to_string();
                    if let Ok(win_odds) = win_odds_text.parse::<f64>() {
                        combinations.push(OddsCombination {
                            first: course_number,
                            second: 0, // 単勝では使用しない
                            third: None,
                            odds: win_odds,
                            is_combined: false,
                            range_text: None, // 単勝では使用しない
                        });
                    }
                    
                    // 複勝オッズ（4列目、"1.5-3.3"形式）
                    let place_odds_text = cells[3].text().collect::<String>().trim().to_string();
                    if place_odds_text.contains("-") {
                        let parts: Vec<&str> = place_odds_text.split("-").collect();
                        if parts.len() == 2 {
                            if let (Ok(min_odds), Ok(max_odds)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                                // 複勝オッズは範囲値をそのまま保存、表示用に平均値も計算
                                let avg_odds = (min_odds + max_odds) / 2.0;
                                combinations.push(OddsCombination {
                                    first: course_number,
                                    second: 1, // 複勝を示すフラグ
                                    third: None,
                                    odds: avg_odds, // 内部計算用の平均値
                                    is_combined: true, // 複勝は範囲値なのでtrueにする
                                    range_text: Some(place_odds_text.clone()), // 表示用の元の範囲文字列
                                });
                            }
                        }
                    }
                }
            }
        }
    } else {
        return Err("単勝・複勝オッズテーブルが見つかりませんでした".into());
    }
    
    println!("単勝・複勝解析完了: {}個のオッズを取得", combinations.len());
    
    Ok(OddsData {
        betting_type: BettingType::WinPlace,
        combinations,
    })
}


#[cfg(test)]
mod odds_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_win_place_odds_from_html_20250726() {
        println!("=== 単勝・複勝オッズデータ解析テスト ===");
        
        // テスト用HTMLファイルを読み込み
        let file_path = "./bort-html/20250726/win_place_odds.html";
        let html_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                println!("HTMLファイルの読み込みに失敗: {}", e);
                println!("まず `cargo test test_fetch_win_place_odds_from_kyoteibiyori` を実行してHTMLファイルを生成してください");
                return;
            }
        };
        
        println!("HTMLファイルサイズ: {} bytes", html_content.len());
        
        // 単勝・複勝オッズデータを解析
        match parse_win_place_odds_from_html(&html_content) {
            Ok(odds_data) => {
                println!("✅ 単勝・複勝オッズ解析成功！");
                println!("ベッティングタイプ: {:?}", odds_data.betting_type);
                println!("総オッズ数: {}", odds_data.combinations.len());
                
                // 各艇のオッズを表示
                for combination in &odds_data.combinations {
                    if combination.second == 0 {
                        println!("{}号艇 単勝: {:.1}倍", combination.first, combination.odds);
                    } else {
                        if let Some(range) = &combination.range_text {
                            println!("{}号艇 複勝: {}倍", combination.first, range);
                        } else {
                            println!("{}号艇 複勝: {:.1}倍（平均）", combination.first, combination.odds);
                        }
                    }
                }
                
                // 基本的な検証
                assert_eq!(odds_data.betting_type, BettingType::WinPlace);
                assert!(odds_data.combinations.len() > 0, "オッズデータが空です");
                assert!(odds_data.combinations.len() <= 12, "オッズ数が異常です（6艇×2種類=最大12個）");
                
                println!("✅ 単勝・複勝オッズ解析テスト完了");
            }
            Err(e) => {
                eprintln!("❌ 単勝・複勝オッズ解析エラー: {}", e);
                panic!("単勝・複勝オッズ解析に失敗: {}", e);
            }
        }
    }

}

use chrono::Local;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)]
pub struct RaceEvent {
    pub venue_id: u32,
    pub venue_name: String,
    pub event_name: String,
    pub grade: String,      // SG, G1, G2, G3, 一般, etc
    pub start_date: String, // 開始日 "2025-09-11"
    pub duration_days: u32, // 開催日数（colspanの値）
}

#[derive(Debug, Serialize, Clone)]
pub struct MonthlySchedule {
    pub year_month: String, // "2025-09"
    pub events: Vec<RaceEvent>,
}

/// 競艇場コードから名称への変換マップ
fn get_venue_name_map() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    map.insert(1, "桐生".to_string());
    map.insert(2, "戸田".to_string());
    map.insert(3, "江戸川".to_string());
    map.insert(4, "平和島".to_string());
    map.insert(5, "多摩川".to_string());
    map.insert(6, "浜名湖".to_string());
    map.insert(7, "蒲郡".to_string());
    map.insert(8, "常滑".to_string());
    map.insert(9, "津".to_string());
    map.insert(10, "三国".to_string());
    map.insert(11, "びわこ".to_string());
    map.insert(12, "住之江".to_string());
    map.insert(13, "尼崎".to_string());
    map.insert(14, "鳴門".to_string());
    map.insert(15, "丸亀".to_string());
    map.insert(16, "児島".to_string());
    map.insert(17, "宮島".to_string());
    map.insert(18, "徳山".to_string());
    map.insert(19, "下関".to_string());
    map.insert(20, "若松".to_string());
    map.insert(21, "芦屋".to_string());
    map.insert(22, "福岡".to_string());
    map.insert(23, "唐津".to_string());
    map.insert(24, "大村".to_string());
    map
}

/// 月間スケジュールHTMLを解析して大会期間情報を取得
pub fn parse_monthly_schedule(
    html_content: &str,
) -> Result<MonthlySchedule, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html_content);
    let venue_name_map = get_venue_name_map();

    println!("月間スケジュール解析開始");

    // 現在の年月を取得
    let now = Local::now();
    let year_month = now.format("%Y-%m").to_string();

    println!("対象年月: {}", year_month);

    // テーブルセレクタ
    let table_selector = Selector::parse("div.table1 table.is-spritedNone1").unwrap();
    let row_selector = Selector::parse("tbody tr.is-fs12").unwrap();
    let venue_link_selector = Selector::parse("th.is-thColor10 a").unwrap();
    let race_cell_selector = Selector::parse("td").unwrap();
    let race_link_selector = Selector::parse("a").unwrap();

    let mut all_events = Vec::new();

    // 各地区のテーブルを処理
    for (table_index, table) in document.select(&table_selector).enumerate() {
        println!("テーブル {} を処理中", table_index + 1);

        // 各競艇場の行を処理
        for row in table.select(&row_selector) {
            // 競艇場情報を取得
            if let Some(venue_link) = row.select(&venue_link_selector).next() {
                let href = venue_link.value().attr("href").unwrap_or("");

                // jcd=XX から競艇場コードを抽出
                if let Some(venue_code) = extract_venue_code(href) {
                    let venue_name = venue_name_map
                        .get(&venue_code)
                        .cloned()
                        .unwrap_or_else(|| format!("競艇場{}", venue_code));

                    // レースイベントセルを処理
                    let race_cells: Vec<_> = row.select(&race_cell_selector).collect();

                    for cell in &race_cells {
                        // colspanがあるセル（大会期間）をチェック
                        if let Some(colspan) = cell.value().attr("colspan") {
                            if let Ok(duration_days) = colspan.parse::<u32>() {
                                // グレードを取得
                                let grade =
                                    extract_grade_from_class(cell.value().classes().collect());

                                // レースリンクから情報を取得
                                if let Some(race_link) = cell.select(&race_link_selector).next() {
                                    let race_href = race_link.value().attr("href").unwrap_or("");
                                    let event_name = race_link.inner_html().trim().to_string();

                                    // 開始日を抽出（hd=YYYYMMDD から）
                                    if let Some(start_date) = extract_start_date(race_href) {
                                        println!(
                                            "大会発見: {} - {} ({}) {}日間 グレード: {}",
                                            venue_name,
                                            event_name,
                                            start_date,
                                            duration_days,
                                            grade
                                        );

                                        all_events.push(RaceEvent {
                                            venue_id: venue_code,
                                            venue_name: venue_name.clone(),
                                            event_name,
                                            grade,
                                            start_date,
                                            duration_days,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("抽出した大会数: {}", all_events.len());

    Ok(MonthlySchedule {
        year_month,
        events: all_events,
    })
}

/// href から競艇場コード(jcd)を抽出
fn extract_venue_code(href: &str) -> Option<u32> {
    for part in href.split(&['?', '&'][..]) {
        if let Some(stripped) = part.strip_prefix("jcd=") {
            if let Ok(code) = stripped.parse::<u32>() {
                return Some(code);
            }
        }
    }
    None
}

/// CSSクラスからグレードを抽出
fn extract_grade_from_class(classes: Vec<&str>) -> String {
    for class in classes {
        match class {
            "is-gradeColorSG" => return "SG".to_string(),
            "is-gradeColorG1" => return "G1".to_string(),
            "is-gradeColorG2" => return "G2".to_string(),
            "is-gradeColorG3" => return "G3".to_string(),
            "is-gradeColorLady" => return "オールレディース".to_string(),
            "is-gradeColorVenus" => return "ヴィーナスシリーズ".to_string(),
            "is-gradeColorRookie" => return "ルーキーシリーズ".to_string(),
            "is-gradeColorTakumi" => return "マスターズリーグ".to_string(),
            "is-gradeColorIppan" => return "一般".to_string(),
            _ => continue,
        }
    }
    "一般".to_string() // デフォルト
}

/// URLから開始日を抽出（hd=YYYYMMDD → YYYY-MM-DD）
fn extract_start_date(href: &str) -> Option<String> {
    for part in href.split(&['?', '&'][..]) {
        if let Some(stripped) = part.strip_prefix("hd=") {
            if stripped.len() == 8 {
                let year = &stripped[0..4];
                let month = &stripped[4..6];
                let day = &stripped[6..8];
                return Some(format!("{}-{}-{}", year, month, day));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_venue_code() {
        assert_eq!(extract_venue_code("/owpc/pc/data/stadium?jcd=01"), Some(1));
        assert_eq!(
            extract_venue_code("/owpc/pc/race/raceindex?jcd=12&hd=20250914"),
            Some(12)
        );
        assert_eq!(
            extract_venue_code("/owpc/pc/race/assen?jcd=24&hd=20250915"),
            Some(24)
        );
        assert_eq!(extract_venue_code("invalid_url"), None);
    }

    #[test]
    fn test_extract_grade_from_class() {
        assert_eq!(extract_grade_from_class(vec!["is-gradeColorSG"]), "SG");
        assert_eq!(extract_grade_from_class(vec!["is-gradeColorG1"]), "G1");
        assert_eq!(extract_grade_from_class(vec!["is-gradeColorIppan"]), "一般");
        assert_eq!(extract_grade_from_class(vec!["other-class"]), "一般");
    }

    #[test]
    fn test_extract_start_date() {
        assert_eq!(
            extract_start_date("/owpc/pc/race/raceindex?jcd=01&hd=20250914"),
            Some("2025-09-14".to_string())
        );
        assert_eq!(
            extract_start_date("/owpc/pc/race/assen?jcd=12&hd=20251225"),
            Some("2025-12-25".to_string())
        );
        assert_eq!(extract_start_date("invalid_url"), None);
    }

    #[test]
    fn test_venue_name_map() {
        let map = get_venue_name_map();
        assert_eq!(map.get(&1), Some(&"桐生".to_string()));
        assert_eq!(map.get(&24), Some(&"大村".to_string()));
        assert_eq!(map.get(&25), None);
    }

    #[test]
    fn test_parse_monthly_schedule_full_structure() {
        // 保存済みの月間スケジュールHTMLを使用してパース結果を確認
        let file_path = "bort-html/monthly_schedule_202509.html";

        // ファイルが存在しない場合はテスト成功として扱う
        if !std::path::Path::new(file_path).exists() {
            println!(
                "✅ HTMLファイルが存在しないためテストをスキップ: {}",
                file_path
            );
            return;
        }

        // HTMLファイルを読み込み
        let html_content = match std::fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                println!("❌ ファイル読み込みエラー: {}", e);
                panic!("HTMLファイル読み込みに失敗");
            }
        };

        println!("📄 HTMLファイル読み込み成功: {} バイト", html_content.len());

        // パース実行
        match parse_monthly_schedule(&html_content) {
            Ok(schedule) => {
                println!("\n🎉 月間スケジュール解析成功!");
                println!("📅 対象年月: {}", schedule.year_month);
                println!("🏁 大会数: {}", schedule.events.len());

                // 構造体全体をデバッグ出力
                println!("\n📊 === 月間スケジュール全体構造 ===");
                println!("{:#?}", schedule);

                // サマリー情報も出力
                println!("\n📈 === サマリー情報 ===");
                let mut venue_map = std::collections::HashMap::new();
                let mut grade_map = std::collections::HashMap::new();

                for event in &schedule.events {
                    // 競艇場ごとの大会数をカウント
                    *venue_map.entry(event.venue_name.clone()).or_insert(0) += 1;
                    // グレードごとの大会数をカウント
                    *grade_map.entry(event.grade.clone()).or_insert(0) += 1;
                }

                println!("\n🏟️  競艇場別大会数:");
                let mut venues: Vec<_> = venue_map.iter().collect();
                venues.sort_by_key(|(name, _)| name.as_str());
                for (venue, count) in venues {
                    println!("  {} : {} 大会", venue, count);
                }

                println!("\n🏆 グレード別大会数:");
                let mut grades: Vec<_> = grade_map.iter().collect();
                grades.sort_by_key(|(grade, _)| grade.as_str());
                for (grade, count) in grades {
                    println!("  {} : {} 大会", grade, count);
                }

                println!("\n🗓️  大会期間の例 (最初の10大会):");
                for (i, event) in schedule.events.iter().take(10).enumerate() {
                    println!(
                        "  {}. {} - {} ({}日間) at {} [{}]",
                        i + 1,
                        event.event_name,
                        event.start_date,
                        event.duration_days,
                        event.venue_name,
                        event.grade
                    );
                }

                // アサーション（基本的な妥当性チェック）
                assert!(!schedule.events.is_empty(), "大会データが空です");
                assert!(!schedule.year_month.is_empty(), "年月データが空です");

                for event in &schedule.events {
                    assert!(
                        event.venue_id >= 1 && event.venue_id <= 24,
                        "無効な競艇場ID: {}",
                        event.venue_id
                    );
                    assert!(!event.venue_name.is_empty(), "競艇場名が空です");
                    assert!(!event.event_name.is_empty(), "大会名が空です");
                    assert!(
                        event.duration_days > 0 && event.duration_days <= 10,
                        "無効な開催期間: {}",
                        event.duration_days
                    );
                    assert!(!event.start_date.is_empty(), "開始日が空です");
                    assert!(!event.grade.is_empty(), "グレードが空です");
                }

                println!("\n✅ 全データの妥当性チェック完了");

                println!("{:#?}", schedule);
            }
            Err(e) => {
                println!("❌ パースエラー: {}", e);
                panic!("月間スケジュールのパースが失敗しました");
            }
        }
    }
}

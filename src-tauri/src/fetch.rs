use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

//　その日のレース情報のhtmlをそろえて保存する関数→単一のレース情報をパースする関数→csvに追記する関数

pub fn fetch_all_race_info(today: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://www.boatrace.jp/owpc/pc/race/index?hd={}", today);

    // HTMLを取得
    let response = get(&url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: {}", url).into());
    }

    let content = response.bytes()?;

    // HTMLをファイルに保存
    let mut file = File::create("test.html")?;
    file.write_all(&content)?;

    // HTMLをStringに変換
    let html_string = std::str::from_utf8(&content)?.to_string();

    Ok(html_string)
}

#[test]
fn test_fetch_all_race_info() {
    let today = "20231001";
    let result = fetch_all_race_info(today);
    if result.is_err() {
        eprintln!("Error fetching: {}", result.unwrap_err());
    } else {
        let content = result.unwrap();
        println!("{:?}", content);
    };
}

use headless_chrome::{Browser, LaunchOptions};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn fetch_shusso_info_from_kyoteibiyori(
    race_no: u32,
    place_no: u32,
    today: &str,
    slider: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    // ベースURLとパラメータを組み立てる
    let url_base = "https://kyoteibiyori.com/race_shusso.php";
    let url = format!(
        "{}?place_no={}&race_no={}&hiduke={}&slider={}",
        url_base, place_no, race_no, today, slider
    );

    // ブラウザを起動
    let browser = Browser::new(LaunchOptions::default())?;
    let tab = browser.new_tab()?;

    // URLに移動
    tab.navigate_to(&url)?.wait_until_navigated()?;

    // スクリーンショットを取得（必要なら）
    // let screenshot_data = tab.capture_screenshot(
    //     headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
    //     None, // 画質の指定（Noneでデフォルト）
    //     None, // クリッピングの指定（Noneで全画面）
    //     true, // from_surface（trueでスクリーン全体をキャプチャ）
    // )?;

    // let mut file = File::create("screenshot.png")?;
    // file.write_all(&screenshot_data)?;

    // 必要な要素がロードされるまで待機
    tab.wait_for_element("#raceBasic")?;

    // ページのHTMLコンテンツを取得
    let content = tab.get_content()?;

    // 必要ならデータをパース
    let data = get_escaped_flame_info(&content)?;

    let file_dir = format!("./bort-html/{}", today);
    fs::create_dir_all(Path::new(&file_dir))?;
    let file_path = format!("./bort-html/{}/biyori.html", today);
    let mut file = File::create(&file_path)?;
    file.write_all(&content.as_bytes())?;

    drop(browser);
    drop(tab);
    Ok(data)
}

pub fn fetch_odds_info_from_kyoteibiyori(
    race_no: u32,
    place_no: u32,
    today: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // オッズデータ取得用のslider=6を固定で使用
    let slider = 6;
    
    // ベースURLとパラメータを組み立てる
    let url_base = "https://kyoteibiyori.com/race_shusso.php";
    let url = format!(
        "{}?place_no={}&race_no={}&hiduke={}&slider={}",
        url_base, place_no, race_no, today, slider
    );

    // ブラウザを起動
    let browser = Browser::new(LaunchOptions::default())?;
    let tab = browser.new_tab()?;

    // URLに移動
    tab.navigate_to(&url)?.wait_until_navigated()?;

    // オッズデータがロードされるまで待機（適切な要素を待つ）
    // TODO: 実際のオッズページの構造を確認してから適切なセレクタに変更
    tab.wait_for_element("body")?;

    // ページのHTMLコンテンツを取得
    let content = tab.get_content()?;

    // オッズHTML用のファイル保存
    let file_dir = format!("./bort-html/{}", today);
    fs::create_dir_all(Path::new(&file_dir))?;
    let file_path = format!("./bort-html/{}/odds.html", today);
    let mut file = File::create(&file_path)?;
    file.write_all(&content.as_bytes())?;

    drop(browser);
    drop(tab);
    Ok(content)
}

// ダミーのデータ処理関数
fn get_escaped_flame_info(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 必要なデータを抽出する処理をここに実装
    Ok(content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_shusso_info_from_kyoteibiyori() {
        // テスト用のパラメータ
        let race_no = 1;
        let place_no = 2;
        let today = "20231001";
        let slider = 1;

        // 関数を呼び出して結果を確認
        match fetch_shusso_info_from_kyoteibiyori(race_no, place_no, today, slider) {
            Ok(data) => {
                println!("Fetched data: {}", data);
                assert!(!data.is_empty(), "データが空だよ～！");
            }
            Err(e) => {
                eprintln!("エラーが発生しました: {}", e);
                assert!(false, "エラーが発生したよ～！");
            }
        }
    }

    #[test]
    fn test_fetch_odds_info_from_kyoteibiyori() {
        // オッズデータ取得テスト用のパラメータ
        let race_no = 1;
        let place_no = 1;
        let today = "20250726";

        println!("オッズデータを取得中: place_no={}, race_no={}, date={}", place_no, race_no, today);

        // 関数を呼び出して結果を確認
        match fetch_odds_info_from_kyoteibiyori(race_no, place_no, today) {
            Ok(html_content) => {
                println!("オッズHTMLを取得しました！HTMLサイズ: {} bytes", html_content.len());
                println!("HTMLファイルは ./bort-html/{}/odds.html に保存されました", today);
                
                // HTMLコンテンツの先頭部分を表示
                let preview = if html_content.len() > 500 {
                    &html_content[..500]
                } else {
                    &html_content
                };
                println!("HTML内容のプレビュー:\n{}", preview);
                
                assert!(!html_content.is_empty(), "オッズHTMLデータが空です！");
            }
            Err(e) => {
                eprintln!("オッズデータ取得でエラーが発生しました: {}", e);
                panic!("オッズデータ取得に失敗しました: {}", e);
            }
        }
    }
}

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
    // オッズ一覧ページ（slider=6）にアクセスし、単勝・複勝タブをクリック
    let slider = 6;
    
    println!("=== 単勝・複勝オッズデータ取得開始 ===");
    println!("競艇場: {}, レース: {}, 日付: {}, slider: {}", place_no, race_no, today, slider);
    
    // ベースURLとパラメータを組み立てる
    let url_base = "https://kyoteibiyori.com/race_shusso.php";
    let url = format!(
        "{}?place_no={}&race_no={}&hiduke={}&slider={}",
        url_base, place_no, race_no, today, slider
    );

    println!("アクセスURL: {}", url);

    // ブラウザを起動
    let browser = Browser::new(LaunchOptions::default())?;
    let tab = browser.new_tab()?;

    // URLに移動
    tab.navigate_to(&url)?.wait_until_navigated()?;

    // オッズページの読み込みを待つ
    tab.wait_for_element("li.btnOdds")?;
    
    // 単勝・複勝タブを探してクリック
    println!("単勝・複勝タブを探しています...");
    let win_place_tab_result = tab.find_element(r#"li.line-left.btnOdds[id="tf"]"#);
    
    match win_place_tab_result {
        Ok(win_place_tab) => {
            println!("単勝・複勝タブを発見、クリックします");
            win_place_tab.click()?;
            
            // クリック後の遷移を待つ
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        Err(_) => {
            println!("単勝・複勝タブが見つかりません。別のセレクタを試します...");
            // より汎用的なセレクタで再試行
            let tabs = tab.find_elements("li.btnOdds")?;
            println!("見つかったタブ数: {}", tabs.len());
            
            for (i, tab_element) in tabs.iter().enumerate() {
                if let Ok(text) = tab_element.get_inner_text() {
                    println!("タブ{}: {}", i, text);
                    if text.contains("単勝") || text.contains("複勝") {
                        println!("単勝・複勝タブを発見、クリックします");
                        tab_element.click()?;
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        break;
                    }
                }
            }
        }
    }

    // ページのHTMLコンテンツを取得
    let content = tab.get_content()?;

    // 単勝・複勝オッズHTML用のファイル保存
    let file_dir = format!("./bort-html/{}", today);
    fs::create_dir_all(Path::new(&file_dir))?;
    let file_path = format!("./bort-html/{}/win_place_odds.html", today);
    let mut file = File::create(&file_path)?;
    file.write_all(&content.as_bytes())?;

    println!("単勝・複勝オッズHTMLを保存: {}", file_path);
    println!("HTMLサイズ: {} bytes", content.len());

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
    fn test_fetch_win_place_odds_from_kyoteibiyori() {
        // 単勝・複勝オッズデータ取得テスト用のパラメータ
        let race_no = 1;
        let place_no = 1;
        let today = "20250726";

        println!("単勝・複勝オッズデータを取得中: place_no={}, race_no={}, date={}", place_no, race_no, today);

        // 関数を呼び出して結果を確認
        match fetch_odds_info_from_kyoteibiyori(race_no, place_no, today) {
            Ok(html_content) => {
                println!("単勝・複勝オッズHTMLを取得しました！HTMLサイズ: {} bytes", html_content.len());
                println!("HTMLファイルは ./bort-html/{}/win_place_odds.html に保存されました", today);
                
                // HTMLコンテンツの先頭部分を表示
                let preview = if html_content.len() > 500 {
                    &html_content[..500]
                } else {
                    &html_content
                };
                println!("HTML内容のプレビュー:\n{}", preview);
                
                // 単勝・複勝オッズページの特徴的な文字列を確認
                if html_content.contains("単勝") || html_content.contains("複勝") {
                    println!("✅ 単勝・複勝オッズページが正常に取得されました");
                } else {
                    println!("⚠️ 単勝・複勝オッズページの内容を確認してください");
                }
                
                assert!(!html_content.is_empty(), "単勝・複勝オッズHTMLデータが空です！");
            }
            Err(e) => {
                eprintln!("単勝・複勝オッズデータ取得でエラーが発生しました: {}", e);
                panic!("単勝・複勝オッズデータ取得に失敗しました: {}", e);
            }
        }
    }
}

use headless_chrome::{Browser, LaunchOptions};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 指定されたURLからHTMLコンテンツをスクレイピングする汎用関数
pub fn scrape_html_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("=== URLからHTMLスクレイピング開始 ===");
    println!("URL: {}", url);

    // ブラウザを起動
    let browser = Browser::new(LaunchOptions::default())?;
    let tab = browser.new_tab()?;

    // URLに移動
    tab.navigate_to(url)?.wait_until_navigated()?;

    // ページの読み込みを待つ（最大5秒）
    std::thread::sleep(std::time::Duration::from_secs(2));

    // ページのHTMLコンテンツを取得
    let content = tab.get_content()?;

    println!("HTML取得成功: {} bytes", content.len());

    // ブラウザを閉じる
    drop(tab);
    drop(browser);

    Ok(content)
}

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
    file.write_all(content.as_bytes())?;

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
    println!(
        "競艇場: {}, レース: {}, 日付: {}, slider: {}",
        place_no, race_no, today, slider
    );

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
    file.write_all(content.as_bytes())?;

    println!("単勝・複勝オッズHTMLを保存: {}", file_path);
    println!("HTMLサイズ: {} bytes", content.len());

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
                panic!("エラーが発生したよ～！");
            }
        }
    }

    #[test]
    fn test_fetch_win_place_odds_from_kyoteibiyori() {
        // 単勝・複勝オッズデータ取得テスト用のパラメータ
        let race_no = 1;
        let place_no = 1;
        let today = "20250726";

        println!(
            "単勝・複勝オッズデータを取得中: place_no={}, race_no={}, date={}",
            place_no, race_no, today
        );

        // 関数を呼び出して結果を確認
        match fetch_odds_info_from_kyoteibiyori(race_no, place_no, today) {
            Ok(html_content) => {
                println!(
                    "単勝・複勝オッズHTMLを取得しました！HTMLサイズ: {} bytes",
                    html_content.len()
                );
                println!(
                    "HTMLファイルは ./bort-html/{}/win_place_odds.html に保存されました",
                    today
                );

                // HTMLコンテンツの先頭部分を表示（文字境界を考慮）
                let preview = html_content.chars().take(500).collect::<String>();
                println!("HTML内容のプレビュー:\n{}", preview);

                // 単勝・複勝オッズページの特徴的な文字列を確認
                if html_content.contains("単勝") || html_content.contains("複勝") {
                    println!("✅ 単勝・複勝オッズページが正常に取得されました");
                } else {
                    println!("⚠️ 単勝・複勝オッズページの内容を確認してください");
                }

                assert!(
                    !html_content.is_empty(),
                    "単勝・複勝オッズHTMLデータが空です！"
                );
            }
            Err(e) => {
                eprintln!("単勝・複勝オッズデータ取得でエラーが発生しました: {}", e);
                panic!("単勝・複勝オッズデータ取得に失敗しました: {}", e);
            }
        }
    }

    #[test]
    fn test_scrape_racelist_from_official_site() {
        // 公式サイトのracelist URLをスクレイピングするテスト
        let url = "https://www.boatrace.jp/owpc/pc/race/racelist?rno=1&jcd=01&hd=20251125";

        println!("=== 公式サイト racelist ページのスクレイピングテスト ===");
        println!("URL: {}", url);

        match scrape_html_from_url(url) {
            Ok(html_content) => {
                println!("✅ HTML取得成功: {} bytes", html_content.len());

                // HTMLの先頭を表示（デバッグ用）
                let preview = html_content.chars().take(500).collect::<String>();
                println!("\n📄 HTML先頭:\n{}", preview);

                // tbody要素の存在確認
                if html_content.contains("<tbody") {
                    println!("\n✅ tbody要素が存在します");

                    // tbody内にデータがあるか確認
                    if let Some(start) = html_content.find("<tbody") {
                        if let Some(end_pos) = html_content[start..].find("</tbody>") {
                            let tbody_content = &html_content[start..start + end_pos.min(1000)];
                            println!("\n🔍 tbody内容（最初の1000文字）:\n{}", tbody_content);

                            // データが含まれているかチェック
                            let has_data = tbody_content.contains("<tr") && tbody_content.contains("<td");
                            if has_data {
                                println!("\n✅ tbodyにデータが含まれています");
                            } else {
                                println!("\n⚠️ tbodyが空です（JavaScriptで動的に生成される可能性）");
                            }
                        }
                    }
                } else {
                    println!("\n❌ tbody要素が見つかりません");
                }

                // レース関連のキーワードチェック
                let keywords = vec!["出走表", "レース", "選手", "艇番"];
                println!("\n🔍 キーワード検索:");
                for keyword in keywords {
                    let found = html_content.contains(keyword);
                    println!("  {} : {}", keyword, if found { "✅ 見つかりました" } else { "❌ 見つかりません" });
                }

                // HTMLファイルとして保存
                let file_path = "./bort-html/racelist_test_20251125.html";
                std::fs::create_dir_all("./bort-html").ok();
                match std::fs::write(file_path, &html_content) {
                    Ok(_) => println!("\n📁 HTMLを保存しました: {}", file_path),
                    Err(e) => println!("\n⚠️ HTML保存失敗: {}", e),
                }

                // 基本的なアサーション
                assert!(!html_content.is_empty(), "HTMLコンテンツが空です");
                assert!(html_content.len() > 1000, "HTMLコンテンツが小さすぎます（{}バイト）", html_content.len());
            }
            Err(e) => {
                eprintln!("\n❌ スクレイピングエラー発生:");
                eprintln!("エラー内容: {}", e);
                eprintln!("エラー詳細: {:?}", e);

                // エラーの種類を特定
                let error_msg = format!("{}", e);
                if error_msg.contains("Chrome") || error_msg.contains("browser") {
                    eprintln!("\n💡 原因: headless_chromeの起動に失敗した可能性があります");
                    eprintln!("   - Chromeがインストールされているか確認してください");
                    eprintln!("   - WSL環境の場合、追加の設定が必要な場合があります");
                } else if error_msg.contains("timeout") || error_msg.contains("Timeout") {
                    eprintln!("\n💡 原因: ページの読み込みがタイムアウトしました");
                    eprintln!("   - ネットワーク接続を確認してください");
                    eprintln!("   - 待機時間を延長する必要があるかもしれません");
                } else if error_msg.contains("navigate") {
                    eprintln!("\n💡 原因: ページへの移動に失敗しました");
                    eprintln!("   - URLが正しいか確認してください");
                    eprintln!("   - サイトがアクセス制限をかけている可能性があります");
                }

                panic!("racelist URLのスクレイピングに失敗: {}", e);
            }
        }
    }
}

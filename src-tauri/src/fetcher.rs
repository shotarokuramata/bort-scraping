/// 月間スケジュールのHTTPフェッチとファイル保存を行うモジュール
/// 月間スケジュールをフェッチしてファイルに保存
pub async fn fetch_and_cache_monthly_schedule() -> Result<(), String> {
    let url = "https://www.boatrace.jp/owpc/pc/race/monthlyschedule";

    println!("月間スケジュールページをフェッチ中: {}", url);

    match reqwest::get(url).await {
        Ok(response) => {
            println!("HTTPレスポンス受信完了: {}", response.status());

            match response.text().await {
                Ok(html) => {
                    println!("HTMLコンテンツサイズ: {} バイト", html.len());

                    // HTMLファイルとして保存（月単位、bort-htmlディレクトリ内）
                    let dir_path = "bort-html";
                    let file_path = format!(
                        "{}/monthly_schedule_{}.html",
                        dir_path,
                        chrono::Local::now().format("%Y%m")
                    );

                    // ディレクトリを作成（存在しない場合）
                    if let Err(e) = std::fs::create_dir_all(dir_path) {
                        return Err(format!("ディレクトリ作成エラー: {}", e));
                    }

                    if let Err(e) = std::fs::write(&file_path, &html) {
                        return Err(format!("HTMLファイル保存エラー: {}", e));
                    }

                    println!("HTMLファイルを保存しました: {}", file_path);

                    // HTMLの最初の200文字を表示（デバッグ用、文字境界を考慮）
                    let preview = html.chars().take(200).collect::<String>();
                    println!("HTML プレビュー: {}", preview);

                    Ok(())
                }
                Err(e) => Err(format!("レスポンステキスト取得エラー: {}", e)),
            }
        }
        Err(e) => Err(format!("HTTP リクエストエラー: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_and_cache_monthly_schedule() {
        let result = fetch_and_cache_monthly_schedule().await;
        println!("{:?}", result);
        assert!(result.is_ok())
    }
}

use crate::models::open_api::{
    ApiDataType, CsvExportRow, PayoutStats, PreviewRecord, PreviewsResponse, ProgramRecord,
    ProgramsResponse, RaceResult, ResultRecord, ResultsResponse,
};
use crate::repositories::sqlite_db::SqliteRepository;
use chrono::Utc;
use std::env;
use std::path::PathBuf;

const BASE_URL: &str = "https://boatraceopenapi.github.io";
const DEFAULT_DB_PATH: &str = "data/open_api.db";

pub struct OpenApiService {
    repository: SqliteRepository,
    http_client: reqwest::Client,
}

impl OpenApiService {
    /// サービスの初期化（デフォルトパスまたは指定パスを使用）
    pub async fn new(db_path: Option<&str>) -> Result<Self, String> {
        // パスの解決：指定されたパスまたはデフォルトパスを絶対パスに変換
        let resolved_path = Self::resolve_db_path(db_path.unwrap_or(DEFAULT_DB_PATH))?;

        let repository = SqliteRepository::new(&resolved_path)
            .await
            .map_err(|e| format!("Failed to initialize SQLite repository: {}", e))?;

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            repository,
            http_client,
        })
    }

    /// 相対パスを絶対パスに変換
    fn resolve_db_path(path: &str) -> Result<String, String> {
        let path_buf = PathBuf::from(path);

        // すでに絶対パスの場合はそのまま返す
        if path_buf.is_absolute() {
            return Ok(path.to_string());
        }

        // 相対パスの場合、カレントディレクトリから解決
        let current_dir = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        let absolute_path = current_dir.join(path_buf);

        // 親ディレクトリが存在しない場合は作成
        if let Some(parent) = absolute_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        absolute_path
            .to_str()
            .ok_or_else(|| "Invalid path".to_string())
            .map(|s| s.to_string())
    }

    /// API の URL を構築
    fn build_url(&self, data_type: ApiDataType, date: &str) -> String {
        let year = &date[0..4];
        format!(
            "{}/{}/v2/{}/{}.json",
            BASE_URL,
            data_type.as_str(),
            year,
            date
        )
    }

    /// API からデータ取得
    pub async fn fetch_data(
        &self,
        data_type: ApiDataType,
        date: &str,
    ) -> Result<String, String> {
        let url = self.build_url(data_type, date);
        println!("🔄 Fetching {} data for date: {} from {}", data_type.as_str(), date, url);

        let response = self.http_client.get(&url)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {} - {}", response.status(), url));
        }

        let json_text = response.text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        println!("✅ Successfully fetched {} data ({} bytes)", data_type.as_str(), json_text.len());
        Ok(json_text)
    }

    /// Previews データをデータベースに保存
    pub async fn save_previews_data(
        &self,
        date: &str,
        json_data: &str,
    ) -> Result<usize, String> {
        println!("💾 Saving previews data for date: {}", date);

        let response: PreviewsResponse = serde_json::from_str(json_data)
            .map_err(|e| format!("JSON parse error: {}", e))?;

        let now = Utc::now().to_rfc3339();
        let mut saved_count = 0;

        for preview in response.previews {
            let venue_code = format!("{:02}", preview.race_stadium_number);
            let record = PreviewRecord {
                id: 0,
                date: date.to_string(),
                venue_code,
                race_number: preview.race_number,
                data_json: serde_json::to_string(&preview)
                    .map_err(|e| format!("Failed to serialize: {}", e))?,
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            self.repository.save_preview(&record).await
                .map_err(|e| format!("Database error: {}", e))?;
            saved_count += 1;
        }

        println!("✅ Saved {} preview records", saved_count);
        Ok(saved_count)
    }

    /// Results データをデータベースに保存
    pub async fn save_results_data(&self, date: &str, json_data: &str) -> Result<usize, String> {
        println!("💾 Saving results data for date: {}", date);

        let response: ResultsResponse = serde_json::from_str(json_data)
            .map_err(|e| format!("JSON parse error: {}", e))?;

        let now = Utc::now().to_rfc3339();
        let mut saved_count = 0;

        for result in response.results {
            let venue_code = format!("{:02}", result.race_stadium_number);
            let record = ResultRecord {
                id: 0,
                date: date.to_string(),
                venue_code,
                race_number: result.race_number,
                data_json: serde_json::to_string(&result)
                    .map_err(|e| format!("Failed to serialize: {}", e))?,
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            self.repository.save_result(&record).await
                .map_err(|e| format!("Database error: {}", e))?;
            saved_count += 1;
        }

        println!("✅ Saved {} result records", saved_count);
        Ok(saved_count)
    }

    /// Programs データをデータベースに保存
    pub async fn save_programs_data(
        &self,
        date: &str,
        json_data: &str,
    ) -> Result<usize, String> {
        println!("💾 Saving programs data for date: {}", date);

        let response: ProgramsResponse = serde_json::from_str(json_data)
            .map_err(|e| format!("JSON parse error: {}", e))?;

        let now = Utc::now().to_rfc3339();
        let mut saved_count = 0;

        for program in response.programs {
            let venue_code = format!("{:02}", program.race_stadium_number);
            let record = ProgramRecord {
                id: 0,
                date: date.to_string(),
                venue_code,
                race_number: program.race_number,
                data_json: serde_json::to_string(&program)
                    .map_err(|e| format!("Failed to serialize: {}", e))?,
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            self.repository.save_program(&record).await
                .map_err(|e| format!("Database error: {}", e))?;
            saved_count += 1;
        }

        println!("✅ Saved {} program records", saved_count);
        Ok(saved_count)
    }

    /// CSV エクスポート
    pub async fn export_to_csv(
        &self,
        output_path: &str,
        data_type: Option<ApiDataType>,
    ) -> Result<usize, String> {
        println!("📊 Exporting to CSV: {} (type: {:?})", output_path, data_type);

        let mut rows = Vec::new();

        match data_type {
            Some(ApiDataType::Previews) => {
                let records = self.repository.get_all_previews().await
                    .map_err(|e| format!("Database error: {}", e))?;
                for r in records {
                    rows.push(CsvExportRow {
                        date: r.date,
                        venue_code: r.venue_code,
                        race_number: r.race_number,
                        data_type: "preview".to_string(),
                        data_json: r.data_json,
                    });
                }
            },
            Some(ApiDataType::Results) => {
                let records = self.repository.get_all_results().await
                    .map_err(|e| format!("Database error: {}", e))?;
                for r in records {
                    rows.push(CsvExportRow {
                        date: r.date,
                        venue_code: r.venue_code,
                        race_number: r.race_number,
                        data_type: "result".to_string(),
                        data_json: r.data_json,
                    });
                }
            },
            Some(ApiDataType::Programs) => {
                let records = self.repository.get_all_programs().await
                    .map_err(|e| format!("Database error: {}", e))?;
                for r in records {
                    rows.push(CsvExportRow {
                        date: r.date,
                        venue_code: r.venue_code,
                        race_number: r.race_number,
                        data_type: "program".to_string(),
                        data_json: r.data_json,
                    });
                }
            },
            None => {
                // 全データをエクスポート
                let previews = self.repository.get_all_previews().await
                    .map_err(|e| format!("Database error: {}", e))?;
                for r in previews {
                    rows.push(CsvExportRow {
                        date: r.date,
                        venue_code: r.venue_code,
                        race_number: r.race_number,
                        data_type: "preview".to_string(),
                        data_json: r.data_json,
                    });
                }

                let results = self.repository.get_all_results().await
                    .map_err(|e| format!("Database error: {}", e))?;
                for r in results {
                    rows.push(CsvExportRow {
                        date: r.date,
                        venue_code: r.venue_code,
                        race_number: r.race_number,
                        data_type: "result".to_string(),
                        data_json: r.data_json,
                    });
                }

                let programs = self.repository.get_all_programs().await
                    .map_err(|e| format!("Database error: {}", e))?;
                for r in programs {
                    rows.push(CsvExportRow {
                        date: r.date,
                        venue_code: r.venue_code,
                        race_number: r.race_number,
                        data_type: "program".to_string(),
                        data_json: r.data_json,
                    });
                }
            }
        }

        let mut wtr = csv::Writer::from_path(output_path)
            .map_err(|e| format!("Failed to create CSV file: {}", e))?;

        for row in &rows {
            wtr.serialize(row)
                .map_err(|e| format!("Failed to write CSV row: {}", e))?;
        }

        wtr.flush()
            .map_err(|e| format!("Failed to flush CSV writer: {}", e))?;

        println!("✅ Exported {} rows to CSV", rows.len());
        Ok(rows.len())
    }

    // ===== 高配当検索機能 =====

    /// 高配当レース検索
    pub async fn search_high_payout_races(
        &self,
        min_payout: i32,
        payout_type: String,
        limit: Option<i32>,
    ) -> Result<Vec<RaceResult>, String> {
        let records = self.repository
            .search_high_payout_races(min_payout, &payout_type, limit)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        records.into_iter()
            .map(|r| serde_json::from_str(&r.data_json)
                .map_err(|e| format!("JSON parse error: {}", e)))
            .collect()
    }

    /// 配当統計情報取得
    pub async fn get_payout_statistics(&self) -> Result<PayoutStats, String> {
        self.repository.get_payout_statistics()
            .await
            .map_err(|e| format!("Database error: {}", e))
    }
}

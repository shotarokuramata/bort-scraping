use crate::models::open_api::{
    ApiDataType, CsvExportRow, PayoutStats, PreviewRecord, PreviewsResponse, ProgramRecord,
    ProgramsResponse, RaceResult, ResultRecord, ResultsResponse, SearchParams,
    RaceRecord, RaceParticipantRecord, RaceCsvRow, RaceParticipantCsvRow, RacePreview, DataSummaryRow,
    BulkFetchSummary, BulkFetchError, OpenApiBulkProgressPayload,
};
use crate::repositories::sqlite_db::SqliteRepository;
use chrono::Utc;
use std::env;
use std::path::PathBuf;
use tauri::Emitter;

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

    /// V3: CSVエクスポート - 2ファイル方式（races.csv + race_participants.csv）
    ///
    /// V3正規化スキーマに基づき、JSONカラムを除外した構造化CSVを出力。
    /// output_dirは出力ディレクトリパス。ファイル名は自動生成（races.csv, race_participants.csv）。
    pub async fn export_to_csv_v3(
        &self,
        output_dir: &str,
    ) -> Result<(usize, usize), String> {
        println!("📊 Exporting V3 normalized data to CSV: {}", output_dir);

        // 1. V3テーブルから全データ取得
        let race_data = self.repository
            .get_all_races_with_participants()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if race_data.is_empty() {
            return Err("No race data found in database. Run V3 migration first.".to_string());
        }

        println!("  📦 Fetched {} races from V3 tables", race_data.len());

        // 2. races.csv 出力
        let races_csv_path = format!("{}/races.csv", output_dir);
        let mut races_writer = csv::Writer::from_path(&races_csv_path)
            .map_err(|e| format!("Failed to create races.csv: {}", e))?;

        let mut race_count = 0;
        for (race, _, _) in &race_data {
            let csv_row = RaceCsvRow::from(race);
            races_writer.serialize(&csv_row)
                .map_err(|e| format!("Failed to write race row: {}", e))?;
            race_count += 1;
        }

        races_writer.flush()
            .map_err(|e| format!("Failed to flush races CSV: {}", e))?;

        println!("  ✅ Exported {} races to {}", race_count, races_csv_path);

        // 3. race_participants.csv 出力
        let participants_csv_path = format!("{}/race_participants.csv", output_dir);
        let mut participants_writer = csv::Writer::from_path(&participants_csv_path)
            .map_err(|e| format!("Failed to create race_participants.csv: {}", e))?;

        let mut participant_count = 0;
        for (race, participants, preview) in &race_data {
            // previewsデータをパース（存在する場合）
            let preview_map = if let Some(preview_record) = preview {
                // JSONをパースしてRacePreview構造体に変換
                match serde_json::from_str::<RacePreview>(&preview_record.data_json) {
                    Ok(preview_data) => Some(preview_data.boats),
                    Err(e) => {
                        eprintln!("⚠️  Failed to parse preview JSON: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            for participant in participants {
                // 参加者の艇番号に対応するpreviewsデータを取得
                let preview_data = preview_map.as_ref().and_then(|boats| {
                    let boat_key = participant.boat_number.to_string();
                    boats.get(&boat_key).map(|boat_info| {
                        (
                            boat_info.racer_weight_adjustment,
                            boat_info.racer_exhibition_time,
                            boat_info.racer_tilt_adjustment,
                        )
                    })
                });

                let csv_row = RaceParticipantCsvRow::from_record(participant, race, preview_data);
                participants_writer.serialize(&csv_row)
                    .map_err(|e| format!("Failed to write participant row: {}", e))?;
                participant_count += 1;
            }
        }

        participants_writer.flush()
            .map_err(|e| format!("Failed to flush participants CSV: {}", e))?;

        println!("  ✅ Exported {} participants to {}", participant_count, participants_csv_path);
        println!("✅ CSV export completed: {} races, {} participants", race_count, participant_count);

        Ok((race_count, participant_count))
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

    // ===== V3検索API: サービス層 =====

    /// 複合条件検索
    pub async fn search_races_advanced(
        &self,
        params: SearchParams,
    ) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
        self.repository
            .search_races_advanced(params)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// 選手番号での検索
    pub async fn search_races_by_racer(
        &self,
        racer_number: i32,
        limit: Option<i32>,
    ) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
        self.repository
            .search_races_by_racer(racer_number, limit)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// 選手名での検索
    pub async fn search_races_by_racer_name(
        &self,
        racer_name: String,
        limit: Option<i32>,
    ) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
        self.repository
            .search_races_by_racer_name(racer_name, limit)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// 級別での検索
    pub async fn search_races_by_class(
        &self,
        racer_class: i32,
        limit: Option<i32>,
    ) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
        self.repository
            .search_races_by_class(racer_class, limit)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// 日付範囲での検索
    pub async fn search_races_by_date_range(
        &self,
        date_from: String,
        date_to: String,
        limit: Option<i32>,
    ) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
        self.repository
            .search_races_by_date_range(date_from, date_to, limit)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// 会場での検索
    pub async fn search_races_by_venue(
        &self,
        venue_code: String,
        limit: Option<i32>,
    ) -> Result<Vec<(RaceRecord, Vec<RaceParticipantRecord>)>, String> {
        self.repository
            .search_races_by_venue(venue_code, limit)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// 日付ごとのデータ取得状況サマリーを取得
    pub async fn get_data_summary(&self) -> Result<Vec<DataSummaryRow>, String> {
        self.repository
            .get_data_summary_by_date()
            .await
            .map_err(|e| format!("Failed to get data summary: {}", e))
    }

    /// 期間を指定してデータを一括取得（Bulk Fetch）
    pub async fn fetch_data_bulk(
        &self,
        window: Option<tauri::Window>,
        data_type: ApiDataType,
        start_date: &str,  // YYYYMMDD形式
        end_date: &str,    // YYYYMMDD形式
    ) -> Result<BulkFetchSummary, String> {
        use chrono::{Duration, NaiveDate};
        use tokio::time::{sleep, Duration as TokioDuration};

        // 日付範囲のパース
        let start = NaiveDate::parse_from_str(start_date, "%Y%m%d")
            .map_err(|e| format!("Invalid start date: {}", e))?;
        let end = NaiveDate::parse_from_str(end_date, "%Y%m%d")
            .map_err(|e| format!("Invalid end date: {}", e))?;

        let total_days = (end - start).num_days() + 1;
        let mut current_date = start;
        let mut current_day = 0;

        let mut success_count = 0;
        let mut error_count = 0;
        let mut skipped_count = 0;
        let mut errors = Vec::new();

        println!(
            "🔄 Starting bulk fetch: {} from {} to {} ({} days)",
            data_type.as_str(),
            start_date,
            end_date,
            total_days
        );

        // 各日付を順次処理
        while current_date <= end {
            current_day += 1;
            let date_str = current_date.format("%Y%m%d").to_string();

            // STEP 1: データが既に存在するかチェック（キャッシュ優先戦略）
            let existing_count = match data_type {
                ApiDataType::Previews => self
                    .repository
                    .count_previews_by_date(&date_str)
                    .await
                    .unwrap_or(0),
                ApiDataType::Results => self
                    .repository
                    .count_results_by_date(&date_str)
                    .await
                    .unwrap_or(0),
                ApiDataType::Programs => self
                    .repository
                    .count_programs_by_date(&date_str)
                    .await
                    .unwrap_or(0),
            };

            if existing_count > 0 {
                // スキップ - 既にDBに存在
                let message = format!("📦 Skipping {} (already in DB)", date_str);
                println!("{}", message);

                if let Some(ref w) = window {
                    w.emit(
                        "open-api-bulk-progress",
                        OpenApiBulkProgressPayload {
                            message,
                            current: current_day as usize,
                            total: total_days as usize,
                            date: date_str.clone(),
                            data_type: data_type.as_str().to_string(),
                            status: "cached".to_string(),
                        },
                    )
                    .ok();
                }

                skipped_count += 1;
                current_date += Duration::days(1);
                continue;
            }

            // STEP 2: APIからデータ取得
            let message = format!("🌐 Fetching {} for {}", data_type.as_str(), date_str);
            println!("{}", message);

            if let Some(ref w) = window {
                w.emit(
                    "open-api-bulk-progress",
                    OpenApiBulkProgressPayload {
                        message: message.clone(),
                        current: current_day as usize,
                        total: total_days as usize,
                        date: date_str.clone(),
                        data_type: data_type.as_str().to_string(),
                        status: "fetching".to_string(),
                    },
                )
                .ok();
            }

            match self.fetch_data(data_type, &date_str).await {
                Ok(json_data) => {
                    // STEP 3: データベースに保存
                    let save_result = match data_type {
                        ApiDataType::Previews => {
                            self.save_previews_data(&date_str, &json_data).await
                        }
                        ApiDataType::Results => {
                            self.save_results_data(&date_str, &json_data).await
                        }
                        ApiDataType::Programs => {
                            self.save_programs_data(&date_str, &json_data).await
                        }
                    };

                    match save_result {
                        Ok(count) => {
                            let message = format!("💾 Saved {} records for {}", count, date_str);
                            println!("{}", message);

                            if let Some(ref w) = window {
                                w.emit(
                                    "open-api-bulk-progress",
                                    OpenApiBulkProgressPayload {
                                        message,
                                        current: current_day as usize,
                                        total: total_days as usize,
                                        date: date_str.clone(),
                                        data_type: data_type.as_str().to_string(),
                                        status: "saved".to_string(),
                                    },
                                )
                                .ok();
                            }

                            success_count += 1;
                        }
                        Err(e) => {
                            let error_msg = format!("Database save error: {}", e);
                            println!("⚠️  {}: {}", date_str, error_msg);
                            errors.push(BulkFetchError {
                                date: date_str.clone(),
                                error_message: error_msg,
                            });
                            error_count += 1;
                        }
                    }
                }
                Err(e) => {
                    // API取得失敗 - ログに記録して継続
                    println!("⚠️  Failed to fetch {}: {}", date_str, e);

                    if let Some(ref w) = window {
                        w.emit(
                            "open-api-bulk-progress",
                            OpenApiBulkProgressPayload {
                                message: format!("❌ Error: {}", e),
                                current: current_day as usize,
                                total: total_days as usize,
                                date: date_str.clone(),
                                data_type: data_type.as_str().to_string(),
                                status: "error".to_string(),
                            },
                        )
                        .ok();
                    }

                    errors.push(BulkFetchError {
                        date: date_str.clone(),
                        error_message: e,
                    });
                    error_count += 1;
                }
            }

            // STEP 4: レート制限（API負荷を避けるため）
            if current_day < total_days {
                sleep(TokioDuration::from_millis(500)).await;
            }

            current_date += Duration::days(1);
        }

        // 最終完了通知
        let completion_message = format!(
            "✅ Bulk fetch completed: {} success, {} skipped, {} errors",
            success_count, skipped_count, error_count
        );
        println!("{}", completion_message);

        if let Some(ref w) = window {
            w.emit(
                "open-api-bulk-progress",
                OpenApiBulkProgressPayload {
                    message: completion_message,
                    current: total_days as usize,
                    total: total_days as usize,
                    date: end_date.to_string(),
                    data_type: data_type.as_str().to_string(),
                    status: "completed".to_string(),
                },
            )
            .ok();
        }

        Ok(BulkFetchSummary {
            total_days: total_days as usize,
            success_count,
            error_count,
            skipped_count,
            errors,
        })
    }
}

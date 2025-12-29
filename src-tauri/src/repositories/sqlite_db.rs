use crate::models::open_api::{
    PayoutStats, PreviewRecord, ProgramRecord, ResultRecord, RaceResult,
    RaceRecord, RaceParticipantRecord, RaceProgram
};
use sqlx::SqlitePool;
use std::collections::HashMap;

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    /// データベース接続とマイグレーションを実行
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let connection_string = format!("sqlite://{}?mode=rwc", db_path);
        let pool = SqlitePool::connect(&connection_string).await?;
        let repo = Self { pool };
        repo.run_migrations().await?;
        Ok(repo)
    }

    /// マイグレーション実行（テーブル作成）
    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        // Previews テーブル作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS previews (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                venue_code TEXT NOT NULL,
                race_number INTEGER NOT NULL,
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(date, venue_code, race_number)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Results テーブル作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                venue_code TEXT NOT NULL,
                race_number INTEGER NOT NULL,
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(date, venue_code, race_number)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Programs テーブル作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS programs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                venue_code TEXT NOT NULL,
                race_number INTEGER NOT NULL,
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(date, venue_code, race_number)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // インデックス作成
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_previews_date ON previews(date)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_results_date ON results(date)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_programs_date ON programs(date)")
            .execute(&self.pool)
            .await?;

        // V2マイグレーション: 高配当検索用カラム追加
        self.migrate_to_v2().await?;

        // V3マイグレーション: 選手情報正規化
        self.migrate_to_v3().await?;

        Ok(())
    }

    // ===== Previews CRUD =====

    /// Preview データを保存（UPSERT）
    pub async fn save_preview(&self, record: &PreviewRecord) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO previews (date, venue_code, race_number, data_json, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(date, venue_code, race_number)
            DO UPDATE SET data_json = excluded.data_json, updated_at = excluded.updated_at
            "#,
        )
        .bind(&record.date)
        .bind(&record.venue_code)
        .bind(record.race_number)
        .bind(&record.data_json)
        .bind(&record.created_at)
        .bind(&record.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }


    /// すべての Previews を取得（CSV エクスポート用）
    pub async fn get_all_previews(&self) -> Result<Vec<PreviewRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, PreviewRecord>(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM previews
             ORDER BY date, venue_code, race_number",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
    }

    // ===== Results CRUD =====

    /// Result データを保存（UPSERT）
    pub async fn save_result(&self, record: &ResultRecord) -> Result<(), sqlx::Error> {
        // JSONパースして配当データを抽出
        let data: RaceResult = serde_json::from_str(&record.data_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // 配当情報の抽出
        let trifecta_payout = data.payouts.trifecta
            .as_ref()
            .and_then(|entries| entries.first())
            .and_then(|e| e.payout);

        let win_payout = data.payouts.win
            .as_ref()
            .and_then(|entries| entries.first())
            .and_then(|e| e.payout);

        let exacta_payout = data.payouts.exacta
            .as_ref()
            .and_then(|entries| entries.first())
            .and_then(|e| e.payout);

        let place_payout_max = data.payouts.place
            .as_ref()
            .and_then(|entries| {
                entries.iter()
                    .filter_map(|e| e.payout)
                    .max()
            });

        // 1着選手の抽出
        let winner = data.boats.iter().find(|b| b.racer_place_number == Some(1));
        let winner_boat_number = winner.and_then(|w| Some(w.racer_boat_number));
        let winner_racer_number = winner.and_then(|w| w.racer_number);

        sqlx::query(
            r#"
            INSERT INTO results (
                date, venue_code, race_number,
                race_wind, race_wind_direction_number, race_wave,
                race_weather_number, race_temperature, race_water_temperature,
                race_technique_number,
                win_payout, place_payout_max, exacta_payout, trifecta_payout,
                winner_boat_number, winner_racer_number,
                data_json, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(date, venue_code, race_number)
            DO UPDATE SET
                race_wind = excluded.race_wind,
                race_wind_direction_number = excluded.race_wind_direction_number,
                race_wave = excluded.race_wave,
                race_weather_number = excluded.race_weather_number,
                race_temperature = excluded.race_temperature,
                race_water_temperature = excluded.race_water_temperature,
                race_technique_number = excluded.race_technique_number,
                win_payout = excluded.win_payout,
                place_payout_max = excluded.place_payout_max,
                exacta_payout = excluded.exacta_payout,
                trifecta_payout = excluded.trifecta_payout,
                winner_boat_number = excluded.winner_boat_number,
                winner_racer_number = excluded.winner_racer_number,
                data_json = excluded.data_json,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&record.date)
        .bind(&record.venue_code)
        .bind(record.race_number)
        .bind(data.race_wind)
        .bind(data.race_wind_direction_number)
        .bind(data.race_wave)
        .bind(data.race_weather_number)
        .bind(data.race_temperature)
        .bind(data.race_water_temperature)
        .bind(data.race_technique_number)
        .bind(win_payout)
        .bind(place_payout_max)
        .bind(exacta_payout)
        .bind(trifecta_payout)
        .bind(winner_boat_number)
        .bind(winner_racer_number)
        .bind(&record.data_json)
        .bind(&record.created_at)
        .bind(&record.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }


    /// すべての Results を取得（CSV エクスポート用）
    pub async fn get_all_results(&self) -> Result<Vec<ResultRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, ResultRecord>(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM results
             ORDER BY date, venue_code, race_number",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
    }

    // ===== Programs CRUD =====

    /// Program データを保存（UPSERT）
    pub async fn save_program(&self, record: &ProgramRecord) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO programs (date, venue_code, race_number, data_json, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(date, venue_code, race_number)
            DO UPDATE SET data_json = excluded.data_json, updated_at = excluded.updated_at
            "#,
        )
        .bind(&record.date)
        .bind(&record.venue_code)
        .bind(record.race_number)
        .bind(&record.data_json)
        .bind(&record.created_at)
        .bind(&record.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }


    /// すべての Programs を取得（CSV エクスポート用）
    pub async fn get_all_programs(&self) -> Result<Vec<ProgramRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, ProgramRecord>(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM programs
             ORDER BY date, venue_code, race_number",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
    }

    // ===== V2マイグレーション: 高配当検索用カラム追加 =====

    /// V2マイグレーション: Resultsテーブルに検索用カラムを追加
    async fn migrate_to_v2(&self) -> Result<(), sqlx::Error> {
        println!("🔄 Running V2 migration: Adding search columns to results table");

        // カラムが既に存在するかチェック（冪等性確保）
        let column_check: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM pragma_table_info('results') WHERE name='trifecta_payout'"
        )
        .fetch_one(&self.pool)
        .await?;

        if column_check.0 > 0 {
            println!("✅ V2 migration already applied, skipping");
            return Ok(());
        }

        // トランザクション開始
        let mut tx = self.pool.begin().await?;

        // Resultsテーブルにカラム追加
        println!("  📝 Adding columns to results table...");
        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_wind REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_wind_direction_number REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_wave REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_weather_number REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_temperature REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_water_temperature REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN race_technique_number REAL;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN win_payout INTEGER;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN place_payout_max INTEGER;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN exacta_payout INTEGER;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN trifecta_payout INTEGER;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN winner_boat_number INTEGER;
            "#
        ).execute(&mut *tx).await?;

        sqlx::query(
            r#"
            ALTER TABLE results ADD COLUMN winner_racer_number INTEGER;
            "#
        ).execute(&mut *tx).await?;

        println!("  ✅ Columns added successfully");

        // 既存データの移行
        println!("  🔄 Migrating existing data...");
        self.migrate_existing_results_data(&mut tx).await?;
        println!("  ✅ Data migration completed");

        // インデックス作成
        println!("  📊 Creating indexes...");
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_results_trifecta_payout ON results(trifecta_payout)")
            .execute(&mut *tx).await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_results_win_payout ON results(win_payout)")
            .execute(&mut *tx).await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_results_exacta_payout ON results(exacta_payout)")
            .execute(&mut *tx).await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_results_venue ON results(venue_code)")
            .execute(&mut *tx).await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_results_date_venue ON results(date, venue_code)")
            .execute(&mut *tx).await?;

        println!("  ✅ Indexes created successfully");

        // コミット
        tx.commit().await?;

        println!("✅ V2 migration completed successfully");
        Ok(())
    }

    /// 既存Resultsデータの移行
    async fn migrate_existing_results_data(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), sqlx::Error> {
        // 全Resultsレコードを取得
        let results: Vec<ResultRecord> = sqlx::query_as(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at FROM results"
        )
        .fetch_all(&mut **tx)
        .await?;

        println!("    📦 Found {} records to migrate", results.len());

        let mut migrated_count = 0;
        let mut error_count = 0;

        for record in results {
            // JSONパース
            let data: RaceResult = match serde_json::from_str(&record.data_json) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("    ⚠️  JSON parse error for record {}: {}", record.id, e);
                    error_count += 1;
                    continue;
                }
            };

            // 配当情報の抽出
            let trifecta_payout = data.payouts.trifecta
                .as_ref()
                .and_then(|entries| entries.first())
                .and_then(|e| e.payout);

            let win_payout = data.payouts.win
                .as_ref()
                .and_then(|entries| entries.first())
                .and_then(|e| e.payout);

            let exacta_payout = data.payouts.exacta
                .as_ref()
                .and_then(|entries| entries.first())
                .and_then(|e| e.payout);

            let place_payout_max = data.payouts.place
                .as_ref()
                .and_then(|entries| {
                    entries.iter()
                        .filter_map(|e| e.payout)
                        .max()
                });

            // 1着選手の抽出
            let winner = data.boats.iter().find(|b| b.racer_place_number == Some(1));
            let winner_boat_number = winner.and_then(|w| Some(w.racer_boat_number));
            let winner_racer_number = winner.and_then(|w| w.racer_number);

            // UPDATE文でデータ更新
            sqlx::query(
                r#"
                UPDATE results SET
                    race_wind = ?,
                    race_wind_direction_number = ?,
                    race_wave = ?,
                    race_weather_number = ?,
                    race_temperature = ?,
                    race_water_temperature = ?,
                    race_technique_number = ?,
                    win_payout = ?,
                    place_payout_max = ?,
                    exacta_payout = ?,
                    trifecta_payout = ?,
                    winner_boat_number = ?,
                    winner_racer_number = ?
                WHERE id = ?
                "#
            )
            .bind(data.race_wind)
            .bind(data.race_wind_direction_number)
            .bind(data.race_wave)
            .bind(data.race_weather_number)
            .bind(data.race_temperature)
            .bind(data.race_water_temperature)
            .bind(data.race_technique_number)
            .bind(win_payout)
            .bind(place_payout_max)
            .bind(exacta_payout)
            .bind(trifecta_payout)
            .bind(winner_boat_number)
            .bind(winner_racer_number)
            .bind(record.id)
            .execute(&mut **tx)
            .await?;

            migrated_count += 1;
        }

        println!("    ✅ Migrated {} records ({} errors)", migrated_count, error_count);
        Ok(())
    }

    // ===== 高配当検索機能 =====

    /// 高配当レース検索
    pub async fn search_high_payout_races(
        &self,
        min_payout: i32,
        payout_type: &str,
        limit: Option<i32>,
    ) -> Result<Vec<ResultRecord>, sqlx::Error> {
        let query = match payout_type {
            "win" => r#"
                SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
                FROM results
                WHERE win_payout >= ?
                ORDER BY win_payout DESC
                LIMIT ?
            "#,
            "trifecta" => r#"
                SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
                FROM results
                WHERE trifecta_payout >= ?
                ORDER BY trifecta_payout DESC
                LIMIT ?
            "#,
            "exacta" => r#"
                SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
                FROM results
                WHERE exacta_payout >= ?
                ORDER BY exacta_payout DESC
                LIMIT ?
            "#,
            "place" => r#"
                SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
                FROM results
                WHERE place_payout_max >= ?
                ORDER BY place_payout_max DESC
                LIMIT ?
            "#,
            _ => return Err(sqlx::Error::RowNotFound),
        };

        sqlx::query_as(query)
            .bind(min_payout)
            .bind(limit.unwrap_or(100))
            .fetch_all(&self.pool)
            .await
    }

    /// 配当統計情報取得
    pub async fn get_payout_statistics(&self) -> Result<PayoutStats, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                AVG(trifecta_payout) as avg_trifecta,
                MAX(trifecta_payout) as max_trifecta,
                AVG(win_payout) as avg_win,
                MAX(win_payout) as max_win
            FROM results
            WHERE trifecta_payout IS NOT NULL
            "#
        )
        .fetch_one(&self.pool)
        .await
    }

    // ===== V3マイグレーション: 選手情報正規化 =====

    /// V3マイグレーション: 選手情報を正規化（racesテーブル + race_participantsテーブル）
    async fn migrate_to_v3(&self) -> Result<(), sqlx::Error> {
        println!("🔄 Running V3 migration: Normalizing racer information");

        // 冪等性チェック
        let table_check: Result<(i64,), _> = sqlx::query_as(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='races'"
        )
        .fetch_one(&self.pool)
        .await;

        if let Ok((count,)) = table_check {
            if count > 0 {
                println!("✅ V3 migration already applied, skipping");
                return Ok(());
            }
        }

        // トランザクション開始
        let mut tx = self.pool.begin().await?;

        // ステップ1: 新テーブル作成
        println!("  📝 Creating new tables...");
        self.create_v3_tables(&mut tx).await?;

        // ステップ2: データ移行
        println!("  🔄 Migrating data from results table...");
        self.migrate_results_to_v3(&mut tx).await?;

        // ステップ3: インデックス作成
        println!("  📊 Creating indexes...");
        self.create_v3_indexes(&mut tx).await?;

        // ステップ4: 整合性検証
        println!("  ✅ Verifying data integrity...");
        self.verify_v3_migration(&mut tx).await?;

        // ステップ5: 旧テーブル削除
        println!("  🗑️  Dropping old results table...");
        sqlx::query("DROP TABLE IF EXISTS results")
            .execute(&mut *tx)
            .await?;

        // コミット
        tx.commit().await?;

        println!("✅ V3 migration completed successfully");
        Ok(())
    }

    /// V3: 新テーブル作成
    async fn create_v3_tables(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), sqlx::Error> {
        // races テーブル作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS races (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                race_date TEXT NOT NULL,
                venue_code TEXT NOT NULL,
                race_number INTEGER NOT NULL,
                race_wind REAL,
                race_wind_direction_number REAL,
                race_wave REAL,
                race_weather_number REAL,
                race_temperature REAL,
                race_water_temperature REAL,
                race_technique_number REAL,
                win_payout INTEGER,
                place_payout_max INTEGER,
                exacta_payout INTEGER,
                quinella_payout INTEGER,
                trifecta_payout INTEGER,
                trio_payout INTEGER,
                winner_boat_number INTEGER,
                winner_racer_number INTEGER,
                race_grade_number INTEGER,
                race_title TEXT,
                race_subtitle TEXT,
                race_distance INTEGER,
                result_data_json TEXT,
                program_data_json TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(race_date, venue_code, race_number)
            )
            "#
        )
        .execute(&mut **tx)
        .await?;

        // race_participants テーブル作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS race_participants (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                race_id INTEGER NOT NULL,
                boat_number INTEGER NOT NULL,
                racer_number INTEGER,
                racer_name TEXT,
                racer_class_number INTEGER,
                racer_branch_number INTEGER,
                racer_birthplace_number INTEGER,
                racer_age INTEGER,
                racer_weight REAL,
                course_number INTEGER,
                start_timing REAL,
                entry_number INTEGER,
                place_number INTEGER,
                decision_hand TEXT,
                flying_count INTEGER,
                late_count INTEGER,
                average_start_timing REAL,
                national_top_1_percent REAL,
                national_top_2_percent REAL,
                national_top_3_percent REAL,
                local_top_1_percent REAL,
                local_top_2_percent REAL,
                local_top_3_percent REAL,
                assigned_motor_number INTEGER,
                assigned_motor_top_2_percent REAL,
                assigned_motor_top_3_percent REAL,
                assigned_boat_number INTEGER,
                assigned_boat_top_2_percent REAL,
                assigned_boat_top_3_percent REAL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (race_id) REFERENCES races(id) ON DELETE CASCADE,
                UNIQUE(race_id, boat_number)
            )
            "#
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// V3: データ移行ロジック
    async fn migrate_results_to_v3(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), sqlx::Error> {
        // 1. 全resultsレコード取得
        let results: Vec<ResultRecord> = sqlx::query_as(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM results ORDER BY date, venue_code, race_number"
        )
        .fetch_all(&mut **tx)
        .await?;

        // 2. 全programsレコード取得（マップ化）
        let programs: Vec<ProgramRecord> = sqlx::query_as(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at FROM programs"
        )
        .fetch_all(&mut **tx)
        .await
        .unwrap_or_default();

        let program_map: HashMap<(String, String, i32), String> = programs
            .into_iter()
            .map(|p| ((p.date, p.venue_code, p.race_number), p.data_json))
            .collect();

        println!("    📦 Found {} results and {} programs to migrate",
            results.len(), program_map.len());

        let mut migrated_races = 0;
        let mut migrated_participants = 0;

        for result_record in results {
            // Results JSONパース
            let result: RaceResult = match serde_json::from_str(&result_record.data_json) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("    ⚠️  Failed to parse result JSON: {}", e);
                    continue;
                }
            };

            // Programs JSONパース（存在する場合）
            let program_data = program_map.get(&(
                result_record.date.clone(),
                result_record.venue_code.clone(),
                result_record.race_number
            ));

            let program: Option<RaceProgram> = program_data
                .and_then(|json| serde_json::from_str(json).ok());

            // 配当情報抽出
            let win_payout = result.payouts.win
                .as_ref().and_then(|e| e.first()).and_then(|p| p.payout);
            let place_payout_max = result.payouts.place
                .as_ref().and_then(|entries| entries.iter().filter_map(|e| e.payout).max());
            let exacta_payout = result.payouts.exacta
                .as_ref().and_then(|e| e.first()).and_then(|p| p.payout);
            let quinella_payout = result.payouts.quinella
                .as_ref().and_then(|e| e.first()).and_then(|p| p.payout);
            let trifecta_payout = result.payouts.trifecta
                .as_ref().and_then(|e| e.first()).and_then(|p| p.payout);
            let trio_payout = result.payouts.trio
                .as_ref().and_then(|e| e.first()).and_then(|p| p.payout);

            // 1着選手抽出
            let winner = result.boats.iter().find(|b| b.racer_place_number == Some(1));
            let winner_boat_number = winner.and_then(|w| Some(w.racer_boat_number));
            let winner_racer_number = winner.and_then(|w| w.racer_number);

            // races テーブルに挿入
            let race_id: i64 = sqlx::query_scalar(
                r#"
                INSERT INTO races (
                    race_date, venue_code, race_number,
                    race_wind, race_wind_direction_number, race_wave,
                    race_weather_number, race_temperature, race_water_temperature,
                    race_technique_number,
                    win_payout, place_payout_max, exacta_payout, quinella_payout,
                    trifecta_payout, trio_payout,
                    winner_boat_number, winner_racer_number,
                    race_grade_number, race_title, race_subtitle, race_distance,
                    result_data_json, program_data_json,
                    created_at, updated_at
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                RETURNING id
                "#
            )
            .bind(&result_record.date)
            .bind(&result_record.venue_code)
            .bind(result_record.race_number)
            .bind(result.race_wind)
            .bind(result.race_wind_direction_number)
            .bind(result.race_wave)
            .bind(result.race_weather_number)
            .bind(result.race_temperature)
            .bind(result.race_water_temperature)
            .bind(result.race_technique_number)
            .bind(win_payout)
            .bind(place_payout_max)
            .bind(exacta_payout)
            .bind(quinella_payout)
            .bind(trifecta_payout)
            .bind(trio_payout)
            .bind(winner_boat_number)
            .bind(winner_racer_number)
            .bind(program.as_ref().and_then(|p| p.race_grade_number))
            .bind(program.as_ref().and_then(|p| p.race_title.clone()))
            .bind(program.as_ref().and_then(|p| p.race_subtitle.clone()))
            .bind(program.as_ref().and_then(|p| p.race_distance))
            .bind(&result_record.data_json)
            .bind(program_data)
            .bind(&result_record.created_at)
            .bind(&result_record.updated_at)
            .fetch_one(&mut **tx)
            .await?;

            migrated_races += 1;

            // race_participants テーブルに挿入（6艇分）
            for boat in &result.boats {
                // Programs データから該当選手を探す
                let program_racer = program.as_ref()
                    .and_then(|p| p.boats.iter().find(|pb|
                        pb.racer_boat_number == Some(boat.racer_boat_number)
                    ));

                sqlx::query(
                    r#"
                    INSERT INTO race_participants (
                        race_id, boat_number,
                        racer_number, racer_name,
                        racer_class_number, racer_branch_number, racer_birthplace_number,
                        racer_age, racer_weight,
                        course_number, start_timing, entry_number,
                        place_number, decision_hand,
                        flying_count, late_count, average_start_timing,
                        national_top_1_percent, national_top_2_percent, national_top_3_percent,
                        local_top_1_percent, local_top_2_percent, local_top_3_percent,
                        assigned_motor_number, assigned_motor_top_2_percent, assigned_motor_top_3_percent,
                        assigned_boat_number, assigned_boat_top_2_percent, assigned_boat_top_3_percent,
                        created_at, updated_at
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(race_id)
                .bind(boat.racer_boat_number)
                .bind(boat.racer_number)
                .bind(boat.racer_name.as_ref())
                .bind(program_racer.and_then(|pr| pr.racer_class_number))
                .bind(program_racer.and_then(|pr| pr.racer_branch_number))
                .bind(program_racer.and_then(|pr| pr.racer_birthplace_number))
                .bind(program_racer.and_then(|pr| pr.racer_age))
                .bind(program_racer.and_then(|pr| pr.racer_weight))
                .bind(boat.racer_course_number)
                .bind(boat.racer_start_timing)
                .bind(None::<i32>) // entry_number（Resultsには無い）
                .bind(boat.racer_place_number)
                .bind(None::<String>) // decision_hand（Resultsには無い）
                .bind(program_racer.and_then(|pr| pr.racer_flying_count))
                .bind(program_racer.and_then(|pr| pr.racer_late_count))
                .bind(program_racer.and_then(|pr| pr.racer_average_start_timing))
                .bind(program_racer.and_then(|pr| pr.racer_national_top_1_percent))
                .bind(program_racer.and_then(|pr| pr.racer_national_top_2_percent))
                .bind(program_racer.and_then(|pr| pr.racer_national_top_3_percent))
                .bind(program_racer.and_then(|pr| pr.racer_local_top_1_percent))
                .bind(program_racer.and_then(|pr| pr.racer_local_top_2_percent))
                .bind(program_racer.and_then(|pr| pr.racer_local_top_3_percent))
                .bind(program_racer.and_then(|pr| pr.racer_assigned_motor_number))
                .bind(program_racer.and_then(|pr| pr.racer_assigned_motor_top_2_percent))
                .bind(program_racer.and_then(|pr| pr.racer_assigned_motor_top_3_percent))
                .bind(program_racer.and_then(|pr| pr.racer_assigned_boat_number))
                .bind(program_racer.and_then(|pr| pr.racer_assigned_boat_top_2_percent))
                .bind(program_racer.and_then(|pr| pr.racer_assigned_boat_top_3_percent))
                .bind(&result_record.created_at)
                .bind(&result_record.updated_at)
                .execute(&mut **tx)
                .await?;

                migrated_participants += 1;
            }
        }

        println!("    ✅ Migrated {} races and {} participants",
            migrated_races, migrated_participants);
        Ok(())
    }

    /// V3: インデックス作成
    async fn create_v3_indexes(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), sqlx::Error> {
        // races テーブルインデックス
        let race_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_races_date ON races(race_date)",
            "CREATE INDEX IF NOT EXISTS idx_races_venue ON races(venue_code)",
            "CREATE INDEX IF NOT EXISTS idx_races_date_venue ON races(race_date, venue_code)",
            "CREATE INDEX IF NOT EXISTS idx_races_trifecta_payout ON races(trifecta_payout)",
            "CREATE INDEX IF NOT EXISTS idx_races_win_payout ON races(win_payout)",
            "CREATE INDEX IF NOT EXISTS idx_races_exacta_payout ON races(exacta_payout)",
            "CREATE INDEX IF NOT EXISTS idx_races_wind ON races(race_wind)",
            "CREATE INDEX IF NOT EXISTS idx_races_wave ON races(race_wave)",
            "CREATE INDEX IF NOT EXISTS idx_races_winner_boat ON races(winner_boat_number)",
            "CREATE INDEX IF NOT EXISTS idx_races_winner_racer ON races(winner_racer_number)",
            "CREATE INDEX IF NOT EXISTS idx_races_grade ON races(race_grade_number)",
        ];

        for sql in race_indexes {
            sqlx::query(sql).execute(&mut **tx).await?;
        }

        // race_participants テーブルインデックス
        let participant_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_participants_race_id ON race_participants(race_id)",
            "CREATE INDEX IF NOT EXISTS idx_participants_racer_number ON race_participants(racer_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_racer_name ON race_participants(racer_name)",
            "CREATE INDEX IF NOT EXISTS idx_participants_class ON race_participants(racer_class_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_branch ON race_participants(racer_branch_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_place ON race_participants(place_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_boat ON race_participants(boat_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_course ON race_participants(course_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_class_place ON race_participants(racer_class_number, place_number)",
            "CREATE INDEX IF NOT EXISTS idx_participants_racer_place ON race_participants(racer_number, place_number)",
        ];

        for sql in participant_indexes {
            sqlx::query(sql).execute(&mut **tx).await?;
        }

        Ok(())
    }

    /// V3: 整合性検証
    async fn verify_v3_migration(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), sqlx::Error> {
        // レコード数チェック
        let old_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM results")
            .fetch_one(&mut **tx)
            .await?;

        let new_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM races")
            .fetch_one(&mut **tx)
            .await?;

        if old_count.0 != new_count.0 {
            return Err(sqlx::Error::Protocol(format!(
                "Race count mismatch: {} results -> {} races",
                old_count.0, new_count.0
            )));
        }

        // 選手レコード数チェック（6倍になっているはず）
        let participant_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM race_participants")
            .fetch_one(&mut **tx)
            .await?;

        let expected_participants = old_count.0 * 6;
        if participant_count.0 != expected_participants {
            return Err(sqlx::Error::Protocol(format!(
                "Participant count mismatch: expected {}, got {}",
                expected_participants, participant_count.0
            )));
        }

        println!("    ✅ Data integrity verified: {} races, {} participants",
            new_count.0, participant_count.0);
        Ok(())
    }
}

use crate::models::open_api::{PayoutStats, PreviewRecord, ProgramRecord, ResultRecord, RaceResult};
use sqlx::SqlitePool;

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
}

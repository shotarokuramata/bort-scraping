use crate::models::open_api::{PreviewRecord, ProgramRecord, ResultRecord};
use sqlx::{Row, SqlitePool};

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

    /// 日付範囲で Previews を取得
    pub async fn get_previews_by_date_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<PreviewRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, PreviewRecord>(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM previews
             WHERE date >= ? AND date <= ?
             ORDER BY date, venue_code, race_number",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
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
        sqlx::query(
            r#"
            INSERT INTO results (date, venue_code, race_number, data_json, created_at, updated_at)
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

    /// 日付範囲で Results を取得
    pub async fn get_results_by_date_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<ResultRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, ResultRecord>(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM results
             WHERE date >= ? AND date <= ?
             ORDER BY date, venue_code, race_number",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
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

    /// 日付範囲で Programs を取得
    pub async fn get_programs_by_date_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<ProgramRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, ProgramRecord>(
            "SELECT id, date, venue_code, race_number, data_json, created_at, updated_at
             FROM programs
             WHERE date >= ? AND date <= ?
             ORDER BY date, venue_code, race_number",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
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
}

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone)]
pub struct Db {
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .connect(database_url)
            .await?;
        
        // Note: We assume the table is created externally or via migration for now to satisfy sqlx macros
        // But we can ensure it exists here too.
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS check_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                check_id TEXT NOT NULL,
                status TEXT NOT NULL,
                message TEXT,
                details JSON,
                executed_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn save_result(
        &self,
        check_id: &str,
        status: &str,
        message: &str,
        details: Option<&Value>,
    ) -> Result<i64> {
        let now = Utc::now();
        let id = sqlx::query!(
            r#"
            INSERT INTO check_results (check_id, status, message, details, executed_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            check_id,
            status,
            message,
            details,
            now
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
    
    pub async fn get_recent_results(&self, limit: i64) -> Result<Vec<(String, String, DateTime<Utc>)>> {
        let rows = sqlx::query!(
            r#"
            SELECT check_id, status, executed_at as "executed_at: DateTime<Utc>"
            FROM check_results
            ORDER BY executed_at DESC
            LIMIT ?
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let results = rows.into_iter().map(|r| (r.check_id, r.status, r.executed_at)).collect();
        Ok(results)
    }
}

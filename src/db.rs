//! Database module.
//!
//! When modifying SQL queries in this file, remember to update the offline query cache:
//! 1. Ensure the database exists and is up to date (`export DATABASE_URL=sqlite:data_dash.db`).
//! 2. Run `cargo sqlx prepare`.
//!
//! This generates/updates `sqlx-data.json` which is required for building in CI or environments without the DB.

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
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS check_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                check_id TEXT NOT NULL,
                status TEXT NOT NULL,
                message TEXT,
                details TEXT,
                executed_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS connection_profiles (
                name TEXT PRIMARY KEY,
                driver TEXT NOT NULL,
                connection_string_template TEXT NOT NULL,
                secret_ref TEXT
            );

            CREATE TABLE IF NOT EXISTS secrets (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
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

    // Connection Profiles
    pub async fn get_connection_profiles(&self) -> Result<Vec<(String, String, String, Option<String>)>> {
        let rows = sqlx::query!(
            r#"SELECT name as "name!", driver as "driver!", connection_string_template as "connection_string_template!", secret_ref FROM connection_profiles"#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| (r.name, r.driver, r.connection_string_template, r.secret_ref)).collect())
    }

    pub async fn save_connection_profile(&self, name: &str, driver: &str, tmpl: &str, secret_ref: Option<&str>) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO connection_profiles (name, driver, connection_string_template, secret_ref) 
               VALUES (?, ?, ?, ?) 
               ON CONFLICT(name) DO UPDATE SET 
               driver=excluded.driver, 
               connection_string_template=excluded.connection_string_template, 
               secret_ref=excluded.secret_ref"#,
            name, driver, tmpl, secret_ref
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_connection_profile(&self, name: &str) -> Result<()> {
        sqlx::query!("DELETE FROM connection_profiles WHERE name = ?", name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Secrets
    pub async fn get_secrets(&self) -> Result<Vec<String>> {
        let rows = sqlx::query!(r#"SELECT key as "key!" FROM secrets"#).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| r.key).collect())
    }

    pub async fn get_secret(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query!("SELECT value FROM secrets WHERE key = ?", key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.value))
    }

    pub async fn save_secret(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO secrets (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            key, value
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_secret(&self, key: &str) -> Result<()> {
        sqlx::query!("DELETE FROM secrets WHERE key = ?", key)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

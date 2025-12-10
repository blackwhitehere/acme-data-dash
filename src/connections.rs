use crate::db::Db;
use crate::secrets::SecretStore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Secret error: {0}")]
    SecretError(#[from] crate::secrets::SecretError),
    #[error("Connection profile not found: {0}")]
    ProfileNotFound(String),
    #[error("Driver error: {0}")]
    DriverError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub name: String,
    pub driver: String,
    pub connection_string_template: String,
    pub connection_type: Option<String>,
    pub secret_ref: Option<String>,
}

pub struct ConnectionManager {
    db: Db,
    secret_store: Arc<dyn SecretStore>,
}

impl ConnectionManager {
    pub fn new(db: Db, secret_store: Arc<dyn SecretStore>) -> Self {
        Self { db, secret_store }
    }

    pub async fn get_connection_string(&self, name: &str) -> Result<String, ConnectionError> {
        let profiles = self
            .db
            .get_connection_profiles()
            .await
            .map_err(|e| ConnectionError::DriverError(e.to_string()))?;

        let profile = profiles
            .into_iter()
            .find(|(n, _, _, _, _)| n == name)
            .ok_or_else(|| ConnectionError::ProfileNotFound(name.to_string()))?;

        let (_, _, tmpl, _conn_type, secret_ref) = profile;
        let mut conn_str = tmpl;

        if let Some(s_ref) = secret_ref {
            let secret = self.secret_store.get_secret(&s_ref).await?;
            // Simple template replacement for now. In a real app, use a proper template engine or specific placeholders.
            conn_str = conn_str.replace("{{PASSWORD}}", &secret);
        }

        Ok(conn_str)
    }
}

#[cfg(test)]
mod tests {
    // Tests removed as they relied on in-memory profiles which are no longer supported directly in ConnectionManager
    // Integration tests would be needed with a real DB or mocked DB.
}

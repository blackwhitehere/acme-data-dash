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
    pub secret_ref: Option<String>,
}

pub struct ConnectionManager {
    profiles: Vec<ConnectionProfile>,
    secret_store: Arc<dyn SecretStore>,
}

impl ConnectionManager {
    pub fn new(profiles: Vec<ConnectionProfile>, secret_store: Arc<dyn SecretStore>) -> Self {
        Self {
            profiles,
            secret_store,
        }
    }

    pub async fn get_connection_string(&self, name: &str) -> Result<String, ConnectionError> {
        let profile = self
            .profiles
            .iter()
            .find(|p| p.name == name)
            .ok_or_else(|| ConnectionError::ProfileNotFound(name.to_string()))?;

        let mut conn_str = profile.connection_string_template.clone();

        if let Some(secret_ref) = &profile.secret_ref {
            let secret = self.secret_store.get_secret(secret_ref).await?;
            // Simple template replacement for now. In a real app, use a proper template engine or specific placeholders.
            conn_str = conn_str.replace("{{PASSWORD}}", &secret);
        }

        Ok(conn_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secrets::MemorySecretStore;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_connection_string_with_secret() {
        let mut secrets = HashMap::new();
        secrets.insert("db_pass".to_string(), "secret123".to_string());
        let secret_store = Arc::new(MemorySecretStore::new(secrets));

        let profiles = vec![ConnectionProfile {
            name: "test_db".to_string(),
            driver: "sqlite".to_string(),
            connection_string_template: "sqlite://user:{{PASSWORD}}@localhost/db".to_string(),
            secret_ref: Some("db_pass".to_string()),
        }];

        let manager = ConnectionManager::new(profiles, secret_store);

        let conn_str = manager.get_connection_string("test_db").await.unwrap();
        assert_eq!(conn_str, "sqlite://user:secret123@localhost/db");
    }

    #[tokio::test]
    async fn test_get_connection_string_missing_profile() {
        let secret_store = Arc::new(MemorySecretStore::new(HashMap::new()));
        let manager = ConnectionManager::new(vec![], secret_store);

        let result = manager.get_connection_string("non_existent").await;
        assert!(matches!(result, Err(ConnectionError::ProfileNotFound(_))));
    }
}

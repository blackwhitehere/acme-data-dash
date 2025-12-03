use async_trait::async_trait;
use std::env;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecretError {
    #[error("Secret not found: {0}")]
    NotFound(String),
    #[error("Secret store error: {0}")]
    StoreError(String),
}

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn get_secret(&self, key: &str) -> Result<String, SecretError>;
}

pub struct EnvVarSecretStore;

#[async_trait]
impl SecretStore for EnvVarSecretStore {
    async fn get_secret(&self, key: &str) -> Result<String, SecretError> {
        env::var(key).map_err(|_| SecretError::NotFound(key.to_string()))
    }
}

use crate::db::Db;

pub struct DbSecretStore {
    db: Db,
}

impl DbSecretStore {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SecretStore for DbSecretStore {
    async fn get_secret(&self, key: &str) -> Result<String, SecretError> {
        self.db.get_secret(key).await
            .map_err(|e| SecretError::StoreError(e.to_string()))?
            .ok_or_else(|| SecretError::NotFound(key.to_string()))
    }
}

pub struct MemorySecretStore {
    secrets: HashMap<String, String>,
}

impl MemorySecretStore {
    #[allow(dead_code)]
    pub fn new(secrets: HashMap<String, String>) -> Self {
        Self { secrets }
    }
}

#[async_trait]
impl SecretStore for MemorySecretStore {
    async fn get_secret(&self, key: &str) -> Result<String, SecretError> {
        self.secrets.get(key).cloned().ok_or_else(|| SecretError::NotFound(key.to_string()))
    }
}

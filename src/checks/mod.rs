pub mod example_check;

use crate::connections::ConnectionManager;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Success,
    Warning,
    Failure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub status: CheckStatus,
    pub message: String,
    pub details: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
}

#[async_trait]
pub trait CheckContext: Send + Sync {
    async fn get_connection_string(
        &self,
        name: &str,
    ) -> Result<String, crate::connections::ConnectionError>;
}

pub struct StandardCheckContext {
    pub connection_manager: Arc<ConnectionManager>,
}

#[async_trait]
impl CheckContext for StandardCheckContext {
    async fn get_connection_string(
        &self,
        name: &str,
    ) -> Result<String, crate::connections::ConnectionError> {
        self.connection_manager.get_connection_string(name).await
    }
}

#[async_trait]
pub trait DataCheck: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Vec<ParameterDefinition>;
    async fn execute(
        &self,
        ctx: &dyn CheckContext,
        params: &HashMap<String, Value>,
    ) -> Result<CheckResult, CheckError>;
}

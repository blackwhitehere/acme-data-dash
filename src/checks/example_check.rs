use super::{CheckContext, CheckError, CheckResult, CheckStatus, DataCheck, ParameterDefinition};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

pub struct ExampleCheck;

#[async_trait]
impl DataCheck for ExampleCheck {
    fn id(&self) -> &str {
        "example_check"
    }

    fn description(&self) -> &str {
        "An example check that verifies connection string availability"
    }

    fn parameters(&self) -> Vec<ParameterDefinition> {
        vec![ParameterDefinition {
            name: "target_date".to_string(),
            description: "The date to check data for".to_string(),
            default: None,
        }]
    }

    async fn execute(
        &self,
        ctx: &dyn CheckContext,
        params: &HashMap<String, Value>,
    ) -> Result<CheckResult, CheckError> {
        let _date = params
            .get("target_date")
            .ok_or(CheckError::ConfigError("Missing target_date".to_string()))?;

        // Simulate getting a connection
        let _conn_str = ctx
            .get_connection_string("default_db")
            .await
            .map_err(|e| CheckError::ExecutionError(e.to_string()))?;

        // In a real check, we would use sqlx or odbc-api here to query the DB.
        // For this example, we just return success.

        Ok(CheckResult {
            status: CheckStatus::Success,
            message: "Data verified successfully".to_string(),
            details: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connections::ConnectionError;

    struct MockContext;

    #[async_trait]
    impl CheckContext for MockContext {
        async fn get_connection_string(&self, name: &str) -> Result<String, ConnectionError> {
            if name == "default_db" {
                Ok("mock_connection_string".to_string())
            } else {
                Err(ConnectionError::ProfileNotFound(name.to_string()))
            }
        }
    }

    #[tokio::test]
    async fn test_example_check_success() {
        let check = ExampleCheck;
        let ctx = MockContext;
        let mut params = HashMap::new();
        params.insert(
            "target_date".to_string(),
            Value::String("2023-10-27".to_string()),
        );

        let result = check.execute(&ctx, &params).await.unwrap();
        assert!(matches!(result.status, CheckStatus::Success));
    }

    #[tokio::test]
    async fn test_example_check_missing_param() {
        let check = ExampleCheck;
        let ctx = MockContext;
        let params = HashMap::new();

        let result = check.execute(&ctx, &params).await;
        assert!(matches!(result, Err(CheckError::ConfigError(_))));
    }
}

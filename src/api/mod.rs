use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use crate::checks::{DataCheck, CheckContext, CheckResult};
use crate::db::Db;

pub struct AppState {
    pub checks: HashMap<String, Arc<dyn DataCheck>>,
    pub check_context: Arc<dyn CheckContext>,
    pub db: Db,
}

#[derive(Deserialize)]
pub struct ExecuteRequest {
    pub params: HashMap<String, Value>,
}

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/checks", get(list_checks))
        .route("/checks/:id/execute", post(execute_check))
        .route("/history", get(get_history))
        .with_state(state)
}

async fn list_checks(State(state): State<Arc<AppState>>) -> Json<Vec<CheckSummary>> {
    let summaries = state.checks.values().map(|c| CheckSummary {
        id: c.id().to_string(),
        description: c.description().to_string(),
        parameters: c.parameters(),
    }).collect();
    Json(summaries)
}

async fn execute_check(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<ExecuteRequest>,
) -> Json<Result<CheckResult, String>> {
    if let Some(check) = state.checks.get(&id) {
        match check.execute(state.check_context.as_ref(), &payload.params).await {
            Ok(result) => {
                // Save to DB
                let status_str = format!("{:?}", result.status);
                let _ = state.db.save_result(&id, &status_str, &result.message, result.details.as_ref()).await;
                Json(Ok(result))
            },
            Err(e) => Json(Err(e.to_string())),
        }
    } else {
        Json(Err("Check not found".to_string()))
    }
}

async fn get_history(State(state): State<Arc<AppState>>) -> Json<Vec<HistoryEntry>> {
    match state.db.get_recent_results(50).await {
        Ok(results) => {
            let history = results.into_iter().map(|(id, status, time)| HistoryEntry {
                check_id: id,
                status,
                executed_at: time.to_rfc3339(),
            }).collect();
            Json(history)
        },
        Err(_) => Json(vec![]),
    }
}

#[derive(Serialize)]
struct CheckSummary {
    id: String,
    description: String,
    parameters: Vec<crate::checks::ParameterDefinition>,
}

#[derive(Serialize)]
struct HistoryEntry {
    check_id: String,
    status: String,
    executed_at: String,
}

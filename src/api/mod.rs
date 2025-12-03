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
use crate::connections::ConnectionProfile;

pub struct AppState {
    pub checks: HashMap<String, Arc<dyn DataCheck>>,
    pub check_context: Arc<dyn CheckContext>,
    pub db: Db,
}

#[derive(Deserialize)]
pub struct ExecuteRequest {
    pub params: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct SaveSecretRequest {
    pub key: String,
    pub value: String,
}

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/checks", get(list_checks))
        .route("/api/checks/:id/execute", post(execute_check))
        .route("/api/history", get(get_history))
        .route("/api/connections", get(list_connections).post(save_connection))
        .route("/api/secrets", get(list_secrets).post(save_secret))
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

async fn list_connections(State(state): State<Arc<AppState>>) -> Json<Vec<ConnectionProfile>> {
    match state.db.get_connection_profiles().await {
        Ok(profiles) => {
            let result = profiles.into_iter().map(|(name, driver, tmpl, secret_ref)| ConnectionProfile {
                name,
                driver,
                connection_string_template: tmpl,
                secret_ref,
            }).collect();
            Json(result)
        },
        Err(_) => Json(vec![]),
    }
}

async fn save_connection(
    State(state): State<Arc<AppState>>,
    Json(profile): Json<ConnectionProfile>,
) -> Json<Result<(), String>> {
    match state.db.save_connection_profile(&profile.name, &profile.driver, &profile.connection_string_template, profile.secret_ref.as_deref()).await {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn list_secrets(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    match state.db.get_secrets().await {
        Ok(secrets) => Json(secrets),
        Err(_) => Json(vec![]),
    }
}

async fn save_secret(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SaveSecretRequest>,
) -> Json<Result<(), String>> {
    match state.db.save_secret(&req.key, &req.value).await {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
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

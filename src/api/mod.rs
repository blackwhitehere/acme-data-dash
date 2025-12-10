use crate::checks::{CheckContext, CheckResult, DataCheck};
use crate::connections::ConnectionProfile;
use crate::db::Db;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

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

#[derive(Deserialize)]
pub struct UnixGroupRequest {
    pub group_name: String,
    pub file_path: String,
    pub permissions: String,
}

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/checks", get(list_checks))
        .route("/api/checks/:id/execute", post(execute_check))
        .route("/api/check-statuses", get(get_check_statuses))
        .route("/api/history", get(get_history))
        .route(
            "/api/connections",
            get(list_connections).post(save_connection),
        )
        .route(
            "/api/connections/:name",
            axum::routing::delete(delete_connection),
        )
        .route("/api/secrets", get(list_secrets).post(save_secret))
        .route("/api/secrets/:key", axum::routing::delete(delete_secret))
        .route(
            "/api/data-sources",
            get(list_data_sources).post(save_data_source),
        )
        .route(
            "/api/data-sources/:name",
            axum::routing::delete(delete_data_source),
        )
        .route(
            "/api/unix-groups",
            get(list_unix_groups).post(save_unix_group),
        )
        .route(
            "/api/unix-groups/:name",
            axum::routing::delete(delete_unix_group),
        )
        .with_state(state)
}

async fn list_checks(State(state): State<Arc<AppState>>) -> Json<Vec<CheckSummary>> {
    let summaries = state
        .checks
        .values()
        .map(|c| CheckSummary {
            id: c.id().to_string(),
            description: c.description().to_string(),
            parameters: c.parameters(),
        })
        .collect();
    Json(summaries)
}

async fn execute_check(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<ExecuteRequest>,
) -> Json<Result<CheckResult, String>> {
    if let Some(check) = state.checks.get(&id) {
        match check
            .execute(state.check_context.as_ref(), &payload.params)
            .await
        {
            Ok(result) => {
                // Save to DB
                let status_str = format!("{:?}", result.status);
                let _ = state
                    .db
                    .save_result(&id, &status_str, &result.message, result.details.as_ref())
                    .await;
                Json(Ok(result))
            }
            Err(e) => Json(Err(e.to_string())),
        }
    } else {
        Json(Err("Check not found".to_string()))
    }
}

async fn get_history(State(state): State<Arc<AppState>>) -> Json<Vec<HistoryEntry>> {
    match state.db.get_recent_results(50).await {
        Ok(results) => {
            let history = results
                .into_iter()
                .map(|(id, status, time)| HistoryEntry {
                    check_id: id,
                    status,
                    executed_at: time.to_rfc3339(),
                })
                .collect();
            Json(history)
        }
        Err(_) => Json(vec![]),
    }
}

async fn get_check_statuses(
    State(state): State<Arc<AppState>>,
) -> Json<HashMap<String, CheckStatus>> {
    match state.db.get_latest_check_statuses().await {
        Ok(statuses) => {
            let result = statuses
                .into_iter()
                .map(|(check_id, (status, executed_at))| {
                    (
                        check_id,
                        CheckStatus {
                            status,
                            executed_at: executed_at.to_rfc3339(),
                        },
                    )
                })
                .collect();
            Json(result)
        }
        Err(_) => Json(HashMap::new()),
    }
}

async fn list_connections(State(state): State<Arc<AppState>>) -> Json<Vec<ConnectionProfile>> {
    match state.db.get_connection_profiles().await {
        Ok(profiles) => {
            let result = profiles
                .into_iter()
                .map(
                    |(name, driver, tmpl, connection_type, secret_ref)| ConnectionProfile {
                        name,
                        driver,
                        connection_string_template: tmpl,
                        connection_type,
                        secret_ref,
                    },
                )
                .collect();
            Json(result)
        }
        Err(_) => Json(vec![]),
    }
}

async fn save_connection(
    State(state): State<Arc<AppState>>,
    Json(profile): Json<ConnectionProfile>,
) -> Json<Result<(), String>> {
    match state
        .db
        .save_connection_profile(
            &profile.name,
            &profile.driver,
            &profile.connection_string_template,
            profile.connection_type.as_deref(),
            profile.secret_ref.as_deref(),
        )
        .await
    {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn delete_connection(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<Result<(), String>> {
    match state.db.delete_connection_profile(&name).await {
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

async fn delete_secret(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Json<Result<(), String>> {
    match state.db.delete_secret(&key).await {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn list_data_sources(State(state): State<Arc<AppState>>) -> Json<Vec<DataSourceInfo>> {
    match state.db.get_data_sources().await {
        Ok(sources) => {
            let result = sources
                .into_iter()
                .map(
                    |(name, connection_name, secret_key, is_valid)| DataSourceInfo {
                        name,
                        connection_name,
                        secret_key,
                        is_valid,
                    },
                )
                .collect();
            Json(result)
        }
        Err(_) => Json(vec![]),
    }
}

async fn save_data_source(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DataSourceRequest>,
) -> Json<Result<(), String>> {
    match state
        .db
        .save_data_source(&req.name, &req.connection_name, &req.secret_key)
        .await
    {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn delete_data_source(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<Result<(), String>> {
    match state.db.delete_data_source(&name).await {
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

#[derive(Serialize)]
struct CheckStatus {
    status: String,
    executed_at: String,
}

#[derive(Serialize)]
struct DataSourceInfo {
    name: String,
    connection_name: String,
    secret_key: String,
    is_valid: bool,
}

#[derive(Deserialize)]
struct DataSourceRequest {
    name: String,
    connection_name: String,
    secret_key: String,
}

#[derive(Serialize)]
struct UnixGroupInfo {
    group_name: String,
    file_path: String,
    permissions: String,
}

async fn list_unix_groups(State(state): State<Arc<AppState>>) -> Json<Vec<UnixGroupInfo>> {
    match state.db.get_unix_groups().await {
        Ok(groups) => {
            let result = groups
                .into_iter()
                .map(|(group_name, file_path, permissions)| UnixGroupInfo {
                    group_name,
                    file_path,
                    permissions,
                })
                .collect();
            Json(result)
        }
        Err(_) => Json(vec![]),
    }
}

async fn save_unix_group(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UnixGroupRequest>,
) -> Json<Result<(), String>> {
    match state
        .db
        .save_unix_group(&req.group_name, &req.file_path, &req.permissions)
        .await
    {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
    }
}

async fn delete_unix_group(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<Result<(), String>> {
    match state.db.delete_unix_group(&name).await {
        Ok(_) => Json(Ok(())),
        Err(e) => Json(Err(e.to_string())),
    }
}

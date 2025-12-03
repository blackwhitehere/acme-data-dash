use acme_data_dash::{
    api::{app_router, AppState},
    checks::{example_check::ExampleCheck, DataCheck, StandardCheckContext},
    connections::{ConnectionManager, ConnectionProfile},
    db::Db,
    secrets::EnvVarSecretStore,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting Acme Data Dash...");

    // 1. Initialize Secrets
    let secret_store = Arc::new(EnvVarSecretStore);

    // 2. Initialize Connections (Dummy config for now)
    let profiles = vec![
        ConnectionProfile {
            name: "default_db".to_string(),
            driver: "sqlite".to_string(),
            connection_string_template: "sqlite::memory:".to_string(),
            secret_ref: None,
        }
    ];
    let connection_manager = Arc::new(ConnectionManager::new(profiles, secret_store));
    let check_context = Arc::new(StandardCheckContext { connection_manager });

    // 3. Initialize DB
    let db = Db::new("sqlite:data_dash.db").await?;

    // 4. Register Checks
    let mut checks: HashMap<String, Arc<dyn DataCheck>> = HashMap::new();
    let example = Arc::new(ExampleCheck);
    checks.insert(example.id().to_string(), example);

    // 5. Build App State
    let state = Arc::new(AppState {
        checks,
        check_context,
        db,
    });

    // 6. Start Server
    let api = app_router(state);
    let app = api.nest_service("/", ServeDir::new("ui/dist"));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

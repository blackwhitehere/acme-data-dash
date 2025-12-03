use acme_data_dash::{
    api::{app_router, AppState},
    checks::{example_check::ExampleCheck, DataCheck, StandardCheckContext},
    connections::ConnectionManager,
    db::Db,
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

    // 1. Initialize DB
    let db = Db::new("sqlite:data_dash.db").await?;

    // 2. Initialize Secrets
    // Using DbSecretStore to allow UI configuration
    let secret_store = Arc::new(acme_data_dash::secrets::DbSecretStore::new(db.clone()));

    // 3. Initialize Connections
    let connection_manager = Arc::new(ConnectionManager::new(db.clone(), secret_store));
    let check_context = Arc::new(StandardCheckContext { connection_manager });

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

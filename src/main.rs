mod api;
mod config;
mod core;
mod protocol;
mod storage;
mod utils;

use crate::storage::backup::restore_storage;
use api::routers::create_router;
use dotenv::dotenv;
use tracing::{error};

const ADDR: &str = "127.0.0.1:7200";
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let app = create_router();
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    if let Err(e) = restore_storage().await {
        error!("Restore from rdb failed: {}", e);
        return;
    }

    if let Err(error) = axum::serve(listener, app).await {
        error!("Failed to start web server: {}", error);
    }
}

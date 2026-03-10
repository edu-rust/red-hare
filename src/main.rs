mod api;
mod config;
mod core;
mod protocol;
mod storage;
mod utils;

use crate::storage::backup::{loop_dump_to_rdb, restore_storage};
use api::routers::create_router;
use dotenv::dotenv;
use tracing::error;

const ADDR: &str = "127.0.0.1:7200";

/**
1. Load environment variables
2. Configure log filtering level
3. Load data from RDB file
4. Start a task to periodically dump memory data to RDB file
5. Start web server since clients interact with the KV server via HTTP protocol
*/

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    if let Err(e) = restore_storage().await {
        error!("Restore from rdb failed: {}", e);
        return;
    }
    tokio::spawn(async move {
        loop_dump_to_rdb().await;
    });
    let app = create_router();
    let listener = match tokio::net::TcpListener::bind(ADDR).await {
        Ok(listener) => listener,
        Err(error) => {
            error!("Failed to bind to {}: {}", ADDR, error);
            return;
        }
    };
    if let Err(error) = axum::serve(listener, app).await {
        error!("Failed to start web server: {}", error);
    }
}

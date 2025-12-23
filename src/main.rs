mod api;
mod config;
mod core;
mod protocol;
mod utils;

use api::routers::create_router;
use dotenv::dotenv;
use tracing::{error, info};
const ADDR: &str = "127.0.0.1:7200";
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    info!("starting server...");
    let app = create_router();
    // 启动服务器 (适用于 axum 0.8.8)
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => {
            info!("server start gracefully");
        }
        Err(error) => {
            error!("sailed to start server on address:{},{}", ADDR, error);
        }
    };
}

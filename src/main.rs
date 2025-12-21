mod api;
mod config;
mod core;
mod utils;

use api::routers::create_router;
use log::{error, info};
const ADDR: &str = "127.0.0.1:7200";
#[tokio::main]
async fn main() {
    let app = create_router();
    // 启动服务器 (适用于 axum 0.8.8)
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => {
            info!("Server stopped gracefully");
        }
        Err(error) => {
            error!("Failed to start server on address:{},{}", ADDR, error);
        }
    };
}

mod api;
mod config;
mod core;
mod utils;
mod protocol;

use api::routers::create_router;
use log::{error, info};
const ADDR: &str = "127.0.0.1:7200";
#[tokio::main]
async fn main() {
    // match load_logging_level() {
    //     Ok(_logging_level) => {
    //         tracing_subscriber::fmt()
    //             .with_max_level(tracing::Level::DEBUG) // 设置日志级别
    //             .init();
    //     }
    //     Err(error) => {
    //         error!("Failed to load logging level:{}", error);
    //     }
    // };
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

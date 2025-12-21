mod api;
mod config;
mod core;
mod utils;

use api::routers::create_router;
use std::string::ToString;
const ADDR: &str = "127.0.0.1:7200";
#[tokio::main]
async fn main() {
    let app = create_router();
    // 启动服务器 (适用于 axum 0.8.8)
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

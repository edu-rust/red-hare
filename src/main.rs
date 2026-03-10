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
 1.加载环境变量
 2.配置日志过滤级别
 3.从rdb文件加载 数据
 4.开启单线程，循环将内存中的数据dump到rdb文件
 5.这里采用的是客户端通过http协议与kv服务器交互，所以这里启动web服务器
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

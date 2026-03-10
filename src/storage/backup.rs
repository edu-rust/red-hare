use crate::storage::rdb::{dump_to_rdb, load_from_rdb};
use std::io::Error;
use std::time::Duration;
use tokio::time;
use tracing::error;

pub async fn restore_storage() -> Result<(), Error> {
    load_from_rdb().await
}

pub async fn loop_dump_to_rdb() {
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        if let Err(e) = dump_to_rdb().await {
            error!("RDB dump failed: {}", e);
        }
    }
}

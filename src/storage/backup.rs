use crate::storage::rdb::{dump_to_rdb, load_from_rdb};
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

/// Restores storage data from the RDB persistence file.
///
/// This function loads previously saved data from the persistence file
/// back into memory, typically called during service startup to restore
/// the previous state.
///
/// # Returns
/// * `Result<(), Error>` - The result of the restore operation
///   - `Ok(())` - Data restoration completed successfully
///   - `Err(Error)` - An error occurred during restoration, such as file not found or data corruption
pub async fn restore_storage() -> Result<(), Error> {
    match load_from_rdb().await {
        Ok(_) => Ok(()),
        Err(e) => {
            // If the file doesn't exist, it's the first run, so return Ok
            if e.kind() == NotFound { Ok(()) } else { Err(e) }
        }
    }
}

/// Periodically dumps storage data to RDB file in an infinite loop.
///
/// This function runs continuously, executing RDB persistence every 60 seconds.
/// It is designed to run as a background task to ensure data durability.
/// Errors during dump are logged but do not interrupt the loop.
///
/// # Parameters
/// None
///
/// # Returns
/// None - This function runs indefinitely and does not return
pub async fn loop_dump_to_rdb() {
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        if let Err(e) = dump_to_rdb().await {
            error!("RDB dump failed: {}", e);
        } else {
            info!("rdb dump completed successfully.");
        }
    }
}

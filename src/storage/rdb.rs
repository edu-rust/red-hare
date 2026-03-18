use crate::config::log::load_rdb_path;
use crate::core::red_hare::{MetaData, RedHare};
use serde::{Deserialize, Serialize};
use std::fs::{File, rename};
use std::io::ErrorKind::Other;
use std::io::{Error, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct Persistence {
    pub key: String,
    pub meta_data: MetaData,
}

pub async fn load_from_rdb() -> Result<(), Error> {
    let log_rdb_path = load_rdb_path().map_err(|e| Error::new(Other, e.to_string()))?;

    let file = File::open(&log_rdb_path)?;
    let data: Vec<Persistence> =
        bincode::deserialize_from(file).map_err(|e| Error::new(Other, e.to_string()))?;
    let mut red_hare = RedHare::get_instance().lock().await;
    for data in data {
        red_hare.put(data.key, data.meta_data);
    }
    Ok(())
}

pub async fn dump_to_rdb() -> Result<(), Error> {
    let log_rdb_path = load_rdb_path().map_err(|e| Error::new(Other, e))?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_rdb_path = format!("{}_{}.rdb", log_rdb_path, timestamp);

    let keys = {
        let red_hare = RedHare::get_instance().lock().await;
        let keys = red_hare.keys_get();
        keys
    };
    if keys.is_empty() {
        return Ok(());
    }
    let mut data_vec = Vec::with_capacity(keys.len());
    for key in keys {
        let meta = {
            let mut red_hare = RedHare::get_instance().lock().await;
            red_hare.get(&key)
        };
        match meta {
            Ok(Some(meta_data)) => data_vec.push(Persistence { key, meta_data }),
            Ok(None) => {}
            Err(e) => error!(
                "failed to get_bytes_value_with_expire for key {}: {}",
                key, e
            ),
        }
    }
    if data_vec.is_empty() {
        return Err(Error::new(Other, "data_vec is empty"));
    }
    write_rdb_file(data_vec, &log_rdb_path)
}

fn write_rdb_file(data: Vec<Persistence>, log_rdb_path: &String) -> Result<(), Error> {
    let serial_data = bincode::serialize(&data).map_err(|e| {
        error!("failed to serialize persistence data with bincode: {}", e);
        Error::new(Other, e.to_string())
    })?;

    let log_rdb_path = Path::new(log_rdb_path);

    let mut log_rdb_path = File::create(&log_rdb_path)?;

    log_rdb_path.write_all(&serial_data)?;

    log_rdb_path.sync_all()?;

    info!("success save_rdb_rdb_file");
    Ok(())
}

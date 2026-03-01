use crate::config::log::load_config;
use crate::core::red_hare::{MetaData, RedHare};
use crate::utils::date::is_after_now;
use serde::{Deserialize, Serialize};
use std::fs::{File, rename};
use std::io::ErrorKind::Other;
use std::io::{Error, Write};
use std::path::Path;
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct Persistence {
    pub key: String,
    pub meta_data: MetaData,
}

pub async fn restore_rdb_file() -> Result<(), Error> {
    let log_rdb_path = load_config()
        .map_err(|e| Error::new(Other, e.to_string()))?
        .logging
        .log_rdb_path;

    let file = File::open(&log_rdb_path)?;
    let data: Vec<Persistence> =
        bincode::deserialize_from(file).map_err(|e| Error::new(Other, e.to_string()))?;
    let mut red_hare = RedHare::get_instance().lock().await;
    for data in data {
        if let Err(e) = red_hare.set_bytes_with_expire(data) {
            error!("set_bytes_with_expire failed: {}", e);
        }
    }
    Ok(())
}

pub async fn save_rdb_data() -> Result<(), Error> {
    let log_rdb_path = load_config()
        .map_err(|e| Error::new(Other, e))?
        .logging
        .log_rdb_path;

    let keys = {
        let red_hare = RedHare::get_instance().lock().await;
        let keys = red_hare.keys_get();
        keys
    };
    if keys.is_empty() {
        return Err(Error::new(Other, "keys is empty"));
    }
    let mut data_vec = Vec::with_capacity(keys.len());

    for key in keys {
        let meta = {
            let red_hare = RedHare::get_instance().lock().await;
            red_hare.get_meta_data_with_expire(&key)
        };
        match meta {
            Ok(Some(meta_data)) => match is_after_now(meta_data.expire_time) {
                Ok(true) => data_vec.push(Persistence { key, meta_data }),
                Ok(false) => {}
                Err(e) => error!("failed to check expiration time for key {}: {}", key, e),
            },
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
    save_rdb_rdb_file(data_vec, &log_rdb_path)
}

fn save_rdb_rdb_file(data: Vec<Persistence>, log_rdb_path: &String) -> Result<(), Error> {
    let serial_data = bincode::serialize(&data).map_err(|e| {
        error!("failed to serialize persistence data with bincode: {}", e);
        Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    let path = Path::new(log_rdb_path);
    let temp_path = path.with_extension("temp");

    let mut temp_rdb_file = File::create(&temp_path)?;

    temp_rdb_file.write_all(&serial_data)?;

    temp_rdb_file.sync_all()?;
    rename(temp_path.clone(), log_rdb_path)?;

    //TODO 这里的sync_all在突然断电以后，有丢失数据的风险.
    let parent_path = temp_path
        .parent()
        .ok_or_else(|| Error::new(std::io::ErrorKind::Other, "parent_path is empty"))?;

    let parent_file = File::open(parent_path)?;

    parent_file.sync_all()?;

    info!("success save_rdb_rdb_file");
    Ok(())
}

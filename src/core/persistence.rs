use crate::config::config::load_config;
use crate::core::red_hare::{MetaData, RedHare};
use crate::utils::date::is_after_now;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Persistence {
    pub key: String,
    pub meta_data: MetaData,
}

pub fn restore_rdb_file() {
    let log_rdb_dir = match load_config() {
        Ok(log_rdb_dir) => log_rdb_dir.logging.log_rdb_dir,
        Err(error) => {
            error!("failed to load_config, error: {}", error);
            return;
        }
    };
    let file = match File::open(&log_rdb_dir) {
        Ok(file) => file,
        Err(error) => {
            error!("failed to open rdb file at {}: {}", log_rdb_dir, error);
            return;
        }
    };
    let data: Vec<Persistence> = match bincode::deserialize_from(file) {
        Ok(data) => data,
        Err(error) => {
            error!("failed to deserialize rdb file, error: {}", error);
            return;
        }
    };
    let red_hare = RedHare::singleton();
    for data in data {
        red_hare.set_bytes_with_expire(data)
    }
}

pub fn save_rdb_file() {
    let red_hare = RedHare::singleton();
    let keys = red_hare.keys_get();
    if keys.is_empty() {
        return;
    }
    let mut data_vec = Vec::with_capacity(keys.len());

    for key in keys {
        match red_hare.get_bytes_value_with_expire(key.clone()) {
            Ok(value) => match value {
                None => {}
                Some(meta_data) => match is_after_now(meta_data.expire_time) {
                    Ok(is_after_now) => {
                        if (is_after_now) {
                            data_vec.push(Persistence { key, meta_data });
                        }
                    }
                    Err(error) => {
                        error!("failed to check expiration time for key {}: {}", key, error);
                    }
                },
            },
            Err(error) => {
                error!(
                    "failed to get_bytes_value_with_expire for key {}: {}",
                    key, error
                );
            }
        }
    }
    if data_vec.is_empty() {
        return;
    }
    save_key_value_pair(data_vec)
}

pub fn save_key_value_pair(data: Vec<Persistence>) {
    let serial_data = match bincode::serialize(&data) {
        Ok(serial_data) => serial_data,
        Err(error) => {
            error!(
                "failed to serialize persistence data with bincode, error: {}",
                error
            );
            return;
        }
    };
    let log_rdb_dir = match load_config() {
        Ok(log_rdb_dir) => log_rdb_dir.logging.log_rdb_dir,
        Err(error) => {
            error!("failed to load_config, error: {}", error);
            return;
        }
    };
    let mut file = match File::create(&log_rdb_dir) {
        Ok(file) => file,
        Err(error) => {
            error!("failed to create rdb file at {}: {}", log_rdb_dir, error);
            return;
        }
    };
    match file.write_all(&serial_data) {
        Ok(_ok) => {
            info!(
                "successfully wrote rdb file with {} records to {}",
                data.len(),
                log_rdb_dir
            );
        }
        Err(error) => {
            error!(
                "failed to write rdb file with {} records to {}: {}",
                data.len(),
                log_rdb_dir,
                error
            );
        }
    }
    drop(file)
}

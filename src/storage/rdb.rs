use crate::config::log::load_log_dir;
use crate::core::red_hare::{MetaData, RedHare};
use crate::utils::common::ensure_dir_exists;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_dir, remove_file, rename};
use std::io::ErrorKind::Other;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct Persistence {
    pub key: String,
    pub meta_data: MetaData,
}
const RDB_BASIC_NAME: &str = "rdb_log_file";

pub async  fn load_from_rdb() -> Result<(), Error> {
    let last_rdb_file = match last_rdb_file_get() {
        Ok(Some(last_rdb_file)) => last_rdb_file,
        Ok(None) => return Ok(()),
        Err(error) => {
            error!("last_rdb_file_get error:{}", error);
            return Ok(());
        }
    };

    let last_rdb_file = File::open(&last_rdb_file).map_err(|e| {
        Error::new(
            Other,
            format!("failed to open RDB file {:?}: {}", last_rdb_file, e),
        )
    })?;
    let data: Vec<Persistence> =
        bincode::deserialize_from(last_rdb_file).map_err(|e| Error::new(Other, e.to_string()))?;
    let mut red_hare = RedHare::get_instance().lock().await;
    for data in data {
        red_hare.put(data.key, data.meta_data, false);
    }
    Ok(())
}

fn last_rdb_file_get() -> Result<Option<PathBuf>, Error> {
    info!("1111111111111");
    let log_dir = load_log_dir().map_err(|e| Error::new(Other, e))?;
    info!("22222222222222");
    ensure_dir_exists(&log_dir)?;
    let all_rdb_file = all_rdb_file_get(&log_dir)?;
    if all_rdb_file.is_empty() {
        return Ok(None);
    }
    if all_rdb_file.is_empty() {
        return Ok(None);
    }

    let last_rdb_file = all_rdb_file.into_iter().max_by_key(|path| {
        path.file_name()
            .and_then(|n| n.to_str())
            .and_then(|name| name.split('_').last())
            .and_then(|ts| ts.replace(".rdb", "").parse::<u64>().ok())
    });
    Ok(last_rdb_file)
}

pub async fn dump_to_rdb() -> Result<(), Error> {
    let log_dir = load_log_dir().map_err(|e| Error::new(Other, e))?;
    ensure_dir_exists(&log_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

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
        return Ok(());
    }
    write_rdb_file(data_vec, log_dir, timestamp)
}

fn write_rdb_file(data: Vec<Persistence>, log_dir: String, time_stamp: u64) -> Result<(), Error> {
    let serial_data = bincode::serialize(&data).map_err(|e| {
        error!("failed to serialize persistence data with bincode: {}", e);
        Error::new(Other, e.to_string())
    })?;
    let expire_rdb_files = all_rdb_file_get(&log_dir)?;

    let temp_path = Path::new(&log_dir).join(format!("{}_temp.rdb", RDB_BASIC_NAME));

    let final_path = Path::new(&log_dir).join(format!("{}_{}.rdb", RDB_BASIC_NAME, time_stamp));
    {
        let mut temp_file_file = File::create(&temp_path)?;
        temp_file_file.write_all(&serial_data)?;
        temp_file_file.sync_all()?;
    }
    rename(&temp_path, &final_path).map_err(|e| {
        let _ = remove_file(&temp_path);
        let _ = remove_file(&final_path);
        e
    })?;
    for file_path in expire_rdb_files {
        if let Err(e) = remove_file(&file_path) {
            error!("failed to delete old RDB file {:?}: {}", file_path, e);
        }
    }
    Ok(())
}

fn all_rdb_file_get(log_dir: &String) -> Result<Vec<PathBuf>, Error> {
    // let log_dir = load_log_dir().map_err(|e| Error::new(Other, e))?;
    // ensure_dir_exists(&log_dir)?;
    let mut path_list = Vec::new();
    for entry in read_dir(log_dir)? {
        let entry = entry?;
        let file_name_str = entry.file_name().to_string_lossy().to_string();
        if file_name_str.contains(RDB_BASIC_NAME) {
            path_list.push(entry.path());
        }
    }
    Ok(path_list)
}

use crate::config::log::load_rdb_path;
use crate::core::red_hare::{MetaData, RedHare};
use serde::{Deserialize, Serialize};
use std::fs::{File, read_dir, remove_file};
use std::io::ErrorKind::Other;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};

#[derive(Serialize, Deserialize)]
pub struct Persistence {
    pub key: String,
    pub meta_data: MetaData,
}

pub async fn load_from_rdb() -> Result<(), Error> {
    let all_rdb_file = all_rdb_file_get()?;
    if all_rdb_file.is_empty() {
        return Ok(());
    }
    let last_rdb_file = last_rdb_file_get(all_rdb_file)?;
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
        red_hare.put(data.key, data.meta_data);
    }
    Ok(())
}

fn last_rdb_file_get(p0: Vec<PathBuf>) -> Result<PathBuf, Error> {
    p0.into_iter()
        .max_by_key(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .and_then(|name| name.split('_').last())
                .and_then(|ts| ts.replace(".rdb", "").parse::<u64>().ok())
        })
        .ok_or_else(|| Error::new(Other, "no RDB files found"))
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
        return Ok(())
    }
    write_rdb_file(data_vec, &log_rdb_path)
}

fn write_rdb_file(data: Vec<Persistence>, log_rdb_path: &String) -> Result<(), Error> {
    let serial_data = bincode::serialize(&data).map_err(|e| {
        error!("failed to serialize persistence data with bincode: {}", e);
        Error::new(Other, e.to_string())
    })?;
    let expire_rdb_files = all_rdb_file_get()?;

    let log_rdb_path = Path::new(log_rdb_path);

    let mut log_rdb_path = File::create(&log_rdb_path)?;

    log_rdb_path.write_all(&serial_data)?;

    log_rdb_path.sync_all()?;

    for file_path in expire_rdb_files {
        if let Err(e) = remove_file(&file_path) {
            error!("failed to delete old RDB file {:?}: {}", file_path, e);
        }
    }
    Ok(())
}
fn all_rdb_file_get() -> Result<Vec<PathBuf>, Error> {
    let log_rdb_path = load_rdb_path().map_err(|e| Error::new(Other, e))?;
    let log_rdb_path = Path::new(&log_rdb_path);
    let file_basic_name = log_rdb_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| Error::new(Other, "invalid file name"))?;

    let parent_path = log_rdb_path
        .parent()
        .ok_or_else(|| Error::new(Other, "parent directory not found"))?;
    if parent_path.is_file() {
        return Err(Error::new(Other, "parent_path is a file"));
    }
    let mut path_list = Vec::new();
    for entry in read_dir(parent_path)? {
        let entry = entry?;
        let file_name_str = entry.file_name().to_string_lossy().to_string();
        if file_name_str.contains(file_basic_name) {
            path_list.push(entry.path());
        }
    }
    Ok(path_list)
}

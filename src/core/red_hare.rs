use crate::config::log::load_log_dir;
use crate::utils::common::{add_nanos, ensure_dir_exists, is_after_now};
use griddle::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::ErrorKind::Other;
use std::io::{BufWriter, Error, Write};
use std::path::Path;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tracing::{error, warn};

pub(crate) const STRING: &str = "string";
pub struct RedHare {
    data: HashMap<String, MetaData>,
    aof_writer: Option<BufWriter<File>>,
}

#[derive(Serialize, Deserialize)]
enum OperateType {
    Put,
    Delete,
}
#[derive(Serialize, Deserialize)]
struct AofOperate {
    operate_type: OperateType,
    key: String,
    meta_data: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetaData {
    pub value: Vec<u8>,
    pub expire_time: Option<u128>,
    pub data_type: String,
}
static INSTANCE: LazyLock<Mutex<RedHare>> = LazyLock::new(|| Mutex::new(RedHare::new()));
impl RedHare {
    fn new() -> Self {
        let aof_writer = (|| -> Option<BufWriter<File>> {
            let log_dir = load_log_dir().ok()?;
            ensure_dir_exists(&log_dir).ok()?;
            let aof_log_file = Path::new(&log_dir).join("aof_log_file.aof");
            let aof_log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(aof_log_file)
                .ok()?;
            Some(BufWriter::with_capacity(64 * 1024, aof_log_file))
        })();
        RedHare {
            data: HashMap::new(),
            aof_writer,
        }
    }

    pub fn get_instance() -> &'static Mutex<RedHare> {
        &INSTANCE
    }

    fn append_aof_log(&mut self, aof_operate: AofOperate) {
        let aof_writer = match self.aof_writer.as_mut() {
            Some(aof_writer) => aof_writer,
            None => {
                warn!("aof_writer is None");
                return;
            }
        };
        let serial_data = bincode::serialize(&aof_operate).map_err(|e| {
            error!("failed to serialize aof_operate data with bincode: {}", e);
            Error::new(Other, e.to_string())
        });
        let serial_data = match serial_data {
            Ok(serial_data) => serial_data,
            Err(error) => {
                error!(
                    "failed to serialize aof_operate data with bincode: {}",
                    error
                );
                return;
            }
        };

        if let Err(error) = aof_writer.write_all(&serial_data) {
            error!("failed to write aof log: {}", error);
        };
        if let Err(error) = aof_writer.flush() {
            error!("failed to flush aof log: {}", error);
        };
    }

    pub fn put(&mut self, k: String, v: MetaData, is_aof: bool) {
        let is_after_now = is_after_now(v.expire_time);
        let is_after_now = match is_after_now {
            Ok(is_after_now) => is_after_now,
            Err(error) => {
                error!("put.is_after_now,error:{}", error);
                return;
            }
        };
        if !is_after_now {
            return;
        }
        self.data.insert(k.clone(), v.clone());
        if is_aof {
            self.append_aof_log(AofOperate {
                operate_type: OperateType::Put,
                key: k,
                meta_data: Some(v),
            });
        }
    }

    pub fn delete(&mut self, k: &String, is_aof: bool) {
        self.data.remove(k);
        if is_aof {
            self.append_aof_log(AofOperate {
                operate_type: OperateType::Delete,
                key: k.clone(),
                meta_data: None,
            });
        }
    }

    pub fn get(&mut self, k: &String) -> Result<Option<MetaData>, String> {
        if k.is_empty() {
            return Err(String::from("key is empty"));
        }
        let meta_data = match self.data.get(k) {
            Some(meta_data) => meta_data,
            None => return Ok(None),
        };

        let is_after_now = is_after_now(meta_data.expire_time)?;
        if !is_after_now {
            self.delete(k, false);
            return Ok(None);
        }
        Ok(Some(MetaData {
            value: meta_data.value.clone(),
            expire_time: meta_data.expire_time,
            data_type: meta_data.data_type.clone(),
        }))
    }
    pub fn keys_get(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

pub fn get_expire_time(expire_time: u128) -> Result<Option<u128>, String> {
    if expire_time == 0 {
        Ok(None)
    } else {
        let ret = (add_nanos(expire_time))?;
        Ok(Some(ret))
    }
}

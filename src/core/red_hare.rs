use crate::config::log::{load_log_dir, load_manifest};
use crate::utils::common::{add_nanos, ensure_dir_exists, is_after_now};
use griddle::HashMap;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io::ErrorKind::Other;
use std::io::{Error};
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tracing::error;

pub(crate) const STRING: &str = "string";
pub struct RedHare {
    data: HashMap<String, MetaData>,
    cur_aof_size: RefCell<u128>,
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
        RedHare {
            data: HashMap::new(),
            // aof_writer,
            cur_aof_size: RefCell::new(0),
        }
    }

    pub fn get_instance() -> &'static Mutex<RedHare> {
        &INSTANCE
    }
    

    pub fn put(&mut self, k: String, v: MetaData, is_aof: bool) -> Result<(), Error> {
        let is_after_now = is_after_now(v.expire_time);
        let is_after_now = match is_after_now {
            Ok(is_after_now) => is_after_now,
            Err(error) => {
                error!("put.is_after_now,error:{}", error);
                return Ok(());
            }
        };
        if !is_after_now {
            return Ok(());
        }
        self.data.insert(k.clone(), v.clone());
        if is_aof {
            
        }
        Ok(())
    }

    pub fn delete(&mut self, k: &String, is_aof: bool) -> Result<(), Error> {
        self.data.remove(k);
        if is_aof {
            // self.append_aof_log(AofOperate {
            //     operate_type: OperateType::Delete,
            //     key: k.clone(),
            //     meta_data: None,
            // })?;
        }
        Ok(())
    }

    pub fn get(&mut self, k: &String) -> Result<Option<MetaData>, Error> {
        if k.is_empty() {
            return Err(Error::new(Other, "key is empty"));
        }
        let meta_data = match self.data.get(k) {
            Some(meta_data) => meta_data,
            None => return Ok(None),
        };

        let is_after_now = is_after_now(meta_data.expire_time)?;
        if !is_after_now {
            self.delete(k, false)?;
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

pub fn get_expire_time(expire_time: u128) -> Result<Option<u128>, Error> {
    if expire_time == 0 {
        Ok(None)
    } else {
        let ret = add_nanos(expire_time)?;
        Ok(Some(ret))
    }
}

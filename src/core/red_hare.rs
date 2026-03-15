use crate::utils::date::{add_nanos, is_after_now};
use griddle::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tracing::error;

pub(crate) const STRING: &str = "string";
pub struct RedHare {
    pub(crate) data: HashMap<String, MetaData>,
}

#[derive(Serialize, Deserialize)]
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
        }
    }

    pub fn get_instance() -> &'static Mutex<RedHare> {
        &INSTANCE
    }

    pub fn put(&mut self, k: String, v: MetaData) {
        let is_after_now = is_after_now(v.expire_time);
        let is_after_now = match is_after_now {
            Ok(is_after_now) => is_after_now,
            Err(error) => {
                error!("put.is_after_now,error:{}", error);
                return;
            }
        };
        if is_after_now {
            self.data.insert(k, v);
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
            self.delete(k);
            return Ok(None);
        }
        Ok(Some(MetaData {
            value: meta_data.value.clone(),
            expire_time: meta_data.expire_time,
            data_type: meta_data.data_type.clone(),
        }))
    }

    pub fn delete(&mut self, k: &String) {
        self.data.remove(k);
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

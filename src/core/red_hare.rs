use crate::storage::persistence::Persistence;
use crate::utils::date::{add_nanos, is_after_now, is_after_now_with_u128};
use griddle::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock};
use tokio::sync::{Mutex};
use tracing::{error, info};

const STRING: &str = "string";
const HASH: &str = "hash";
pub struct RedHare {
    data: HashMap<String, MetaData>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub value: Vec<u8>,
    pub expire_time: Option<u128>,
    pub data_type: String,
}
static INSTANCE: LazyLock<Mutex<RedHare>> = LazyLock::new(|| { Mutex::new(RedHare::new()) });
impl RedHare {
    fn new() -> Self {
        RedHare {
            data: HashMap::new(),
        }
    }

    pub fn get_instance() -> &'static Mutex<RedHare> {
        &INSTANCE
    }

    fn insert(&mut self, k: String, v: MetaData) {
        self.data.insert(k, v);
    }
    pub fn keys_get(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
    pub fn set_bytes_with_expire(&mut self, persistence: Persistence) {
        let meta_data = persistence.meta_data;
        match meta_data.expire_time {
            None => self.insert(persistence.key, meta_data),
            Some(expire_time) => match is_after_now_with_u128(expire_time) {
                Ok(is_after_now) => {
                    if is_after_now {
                        self.insert(persistence.key, meta_data);
                    }
                }
                Err(error) => {
                    error!(
                        "failed to set_bytes_with_expire.validate expiration time: {}",
                        error
                    );
                    return;
                }
            },
        };
    }

    pub fn get_meta_data_with_expire(&self, k: &String) -> Result<Option<MetaData>, String> {
        if k.is_empty() {
            return Err(String::from("key is empty"));
        }
        let meta_data = match self.data.get(k) {
            Some(meta_data) => meta_data,
            None => return Ok(None),
        };
        Ok(Some(MetaData {
            value: meta_data.value.clone(),
            expire_time: meta_data.expire_time,
            data_type: meta_data.data_type.clone(),
        }))
    }
}

// hash 操作
impl RedHare {
    pub fn h_set_string(&self, k: String, field: String, v: Vec<u8>) {}

    pub fn h_set_string_with_expire(&self, k: String, field: String, v: Vec<u8>) {}

    pub fn h_get_string(&self, k: String, field: String) {}
    pub fn h_get_all_string(&self, k: String) {}
}

//字符串操作
impl RedHare {
    pub fn set_string(&mut self, k: &String, v: String) -> Result<bool, String> {
        if k.is_empty() {
            return Err(String::from("key is empty"));
        }
        let value = v.into_bytes();
        self.insert(
            k.clone(),
            MetaData {
                value,
                expire_time: None,
                data_type: String::from(STRING),
            },
        );
        Ok(true)
    }
    pub fn set_string_with_expire(
        &mut self,
        k: String,
        v: String,
        expire_time: u128,
    ) -> Result<bool, String> {
        if k.is_empty() {
            return Err(String::from("key is empty"));
        }
        let expire_time = add_nanos(expire_time)?;
        let value = v.into_bytes();
        self.insert(
            k,
            MetaData {
                value,
                expire_time: Some(expire_time),
                data_type: String::from(STRING),
            },
        );
        Ok(true)
    }

    pub fn get_string(&mut self, k: &String) -> Result<Option<String>, String> {
        let data = self.get_meta_data_with_expire(k);
        let data = match data {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        let data = match data {
            None => return Ok(None),
            Some(data) => data,
        };
        if data.data_type != STRING {
            return Err(String::from("data type is not string"));
        };
        let is_after_now = is_after_now(data.expire_time)?;
        if !is_after_now {
            info!("key: {} is expired", k);
            drop(data); // 释放锁后再删除
            self.data.remove(k);
            return Ok(None);
        }
        String::from_utf8(data.value)
            .map(|s| Some(s))
            .map_err(|e| e.to_string())
    }
}

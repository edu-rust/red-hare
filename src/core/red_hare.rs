use crate::core::data_utils::add_nanos;
use dashmap::DashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RedHare {
    store: DashMap<String, MetaData>,
}

struct MetaData {
    value: Vec<u8>,
    expire_time: Option<u128>,
}

impl RedHare {
    pub fn new() -> Self {
        RedHare {
            store: DashMap::new(),
        }
    }

    pub fn set_string(&self, k: String, v: String) -> Result<bool, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let value = v.into_bytes();
        self.store.insert(
            k,
            MetaData {
                value,
                expire_time: None,
            },
        );
        Ok(true)
    }
    pub fn set_string_with_expire(
        &self,
        k: String,
        v: String,
        expire_time: u128,
    ) -> Result<bool, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let expire_time = add_nanos(expire_time)?;
        let value = v.into_bytes();
        self.store.insert(
            k,
            MetaData {
                value,
                expire_time: Some(expire_time),
            },
        );
        Ok(true)
    }

    pub fn get_string(&self, k: String) -> Result<Option<String>, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let meta_data = match self.store.get(&k) {
            Some(data) => data,
            None => return Ok(None),
        };

        let value = meta_data.value.clone();
        if value.is_empty() {
            return Ok(None);
        }

        if let Some(expire_time) = meta_data.expire_time {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_nanos();

            if current_time > expire_time {
                drop(meta_data); // 释放锁后再删除
                self.store.remove(&k);
                return Ok(None);
            }
        }

        match String::from_utf8(value) {
            Ok(data) => Ok(Some(data)),
            Err(e) => Err(e.to_string()),
        }
    }
}

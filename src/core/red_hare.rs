use crate::utils::date::add_nanos;
use dashmap::DashMap;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RedHare {
    data: DashMap<String, MetaData>,
}

struct MetaData {
    value: Vec<u8>,
    expire_time: Option<u128>,
}

impl RedHare {
    fn new() -> Self {
        RedHare {
            data: DashMap::new(),
        }
    }

    pub fn single_instance() -> &'static RedHare {
        static INSTANCE: OnceLock<RedHare> = OnceLock::new();
        INSTANCE.get_or_init(|| RedHare::new())
    }

    pub fn key_set(&self) -> Vec<String> {
        self.data.iter().map(|entry| entry.key().clone()).collect()
    }
    pub fn set_string(&self, k: String, v: String) -> Result<bool, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let value = v.into_bytes();
        self.data.insert(
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
        self.data.insert(
            k,
            MetaData {
                value,
                expire_time: Some(expire_time),
            },
        );
        Ok(true)
    }

    pub fn get_bytes_value(&self, k: String) -> Result<Option<Vec<u8>>, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let meta_data = match self.data.get(&k) {
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
                self.data.remove(&k);
                return Ok(None);
            }
        }
        Ok(Some(value))
    }

    pub fn get_string(&self, k: String) -> Result<Option<String>, String> {
        let data = self.get_bytes_value(k);
        let data = match data {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        let data = match data {
            None => return Ok(None),
            Some(data) => data,
        };

        String::from_utf8(data)
            .map(|s| Some(s))
            .map_err(|e| e.to_string())
    }
}

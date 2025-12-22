use crate::core::persistence::Persistence;
use crate::utils::date::{add_nanos, is_after_now, is_after_now_with_u128};
use dashmap::DashMap;
use log::{error};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub struct RedHare {
    data: DashMap<String, MetaData>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub value: Vec<u8>,
    pub expire_time: Option<u128>,
}

impl RedHare {
    fn new() -> Self {
        RedHare {
            data: DashMap::new(),
        }
    }

    pub fn singleton() -> &'static RedHare {
        static INSTANCE: OnceLock<RedHare> = OnceLock::new();
        INSTANCE.get_or_init(|| RedHare::new())
    }

    pub fn keys_get(&self) -> Vec<String> {
        self.data.iter().map(|entry| entry.key().clone()).collect()
    }
    pub fn set_bytes_with_expire(&self, persistence: Persistence) {
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
    fn insert(&self, k: String, v: MetaData) {
        self.data.insert(k, v);
    }

    pub fn set_string(&self, k: String, v: String) -> Result<bool, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let value = v.into_bytes();
        self.insert(
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
        self.insert(
            k,
            MetaData {
                value,
                expire_time: Some(expire_time),
            },
        );
        Ok(true)
    }

    pub fn get_bytes_value_with_expire(&self, k: String) -> Result<Option<MetaData>, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let meta_data = match self.data.get(&k) {
            Some(meta_data) => meta_data,
            None => return Ok(None),
        };
        Ok(Some(MetaData {
            value: meta_data.value.clone(),
            expire_time: meta_data.expire_time,
        }))
    }

    pub fn get_bytes_value(&self, k: String) -> Result<Option<Vec<u8>>, String> {
        if k.is_empty() {
            return Err("key is empty".to_string());
        }
        let meta_data = match self.data.get(&k) {
            Some(meta_data) => meta_data,
            None => return Ok(None),
        };
        let value = meta_data.value.clone();
        if value.is_empty() {
            return Ok(None);
        }
        let is_after_now = is_after_now(meta_data.expire_time)?;
        if !is_after_now {
            drop(meta_data); // 释放锁后再删除
            self.data.remove(&k);
            return Ok(None);
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

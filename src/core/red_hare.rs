use crate::storage::persistence::Persistence;
use crate::utils::date::is_after_now_with_u128;
use griddle::HashMap;
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::sync::LazyLock;
use tokio::sync::Mutex;

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

    pub fn insert(&mut self, k: String, v: MetaData) {
        self.data.insert(k, v);
    }
    pub fn keys_get(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
    pub fn set_bytes(&mut self, persistence: Persistence) -> Result<(), Error> {
        let meta_data = persistence.meta_data;
        match meta_data.expire_time {
            None => self.insert(persistence.key, meta_data),
            Some(expire_time) => match is_after_now_with_u128(expire_time) {
                Ok(is_after_now) => {
                    if is_after_now {
                        self.insert(persistence.key, meta_data);
                    }
                }
                Err(error) => return Err(Error::other(error)),
            },
        };
        Ok(())
    }

    pub fn get_meta_data(&self, k: &String) -> Result<Option<MetaData>, String> {
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


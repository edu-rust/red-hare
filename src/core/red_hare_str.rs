use crate::core::red_hare::{MetaData, RedHare, STRING};
use crate::utils::date::{add_nanos, is_after_now};
use tracing::info;

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
                data_type: String::from(crate::core::red_hare::STRING),
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
        let data = self.get_meta_data(k);
        let data = match data {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        let data = match data {
            None => return Ok(None),
            Some(data) => data,
        };
        if data.data_type != crate::core::red_hare::STRING {
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

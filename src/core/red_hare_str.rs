use crate::core::red_hare::{get_expire_time, MetaData, RedHare, STRING};
use crate::utils::date::{add_nanos, is_after_now};

impl RedHare {

    //传入的expire_time如果是0,则永不失效
    pub fn set_string_with_expire(
        &mut self,
        k: String,
        v: String,
        expire_time: u128,
    ) -> Result<bool, String> {
        if k.is_empty() {
            return Err(String::from("key is empty"));
        }
        let expire_time = get_expire_time(expire_time)?;
        let value = v.into_bytes();
        self.put(
            k,
            MetaData {
                value,
                expire_time,
                data_type: String::from(STRING),
            },
        );
        Ok(true)
    }

    pub fn get_string(&mut self, k: &String) -> Result<Option<String>, String> {
        let data = self.get(k);
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
        String::from_utf8(data.value)
            .map(|s| Some(s))
            .map_err(|e| e.to_string())
    }
}

use crate::core::red_hare::RedHare;
use tokio::task::spawn_blocking;

pub fn set_string(k: String, v: String) -> Result<bool, String> {
    let red_hare = RedHare::singleton();
    let ret = red_hare.set_string(k, v)?;
    Ok(ret)
}

pub fn set_string_with_expire(k: String, v: String, expire_time: u128) -> Result<bool, String> {
    let red_hare = RedHare::singleton();
    let ret = red_hare.set_string_with_expire(k, v, expire_time)?;
    Ok(ret)
}

pub fn get_string(k: String) -> Result<Option<String>, String> {
    let red_hare = RedHare::singleton();
    let ret = red_hare.get_string(k)?;
    Ok(ret)
}

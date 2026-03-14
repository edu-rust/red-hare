use crate::core::red_hare::RedHare;
use tracing::info;

pub async fn set_string_with_expire(
    k: String,
    v: String,
    expire_time: u128,
) -> Result<bool, String> {
    let mut red_hare = RedHare::get_instance().lock().await;
    let ret = red_hare.set_string_with_expire(k.clone(), v, expire_time)?;
    Ok(ret)
}

pub async fn get_string(k: String) -> Result<Option<String>, String> {
    let mut red_hare = RedHare::get_instance().lock().await;
    let ret = red_hare.get_string(&k)?;
    Ok(ret)
}

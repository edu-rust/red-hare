use crate::core::red_hare::RedHare;
use tokio::task::spawn_blocking;

pub async fn set_string(k: String, v: String) -> Result<bool, String> {
    let red_hare = RedHare::singleton();
    let ret = spawn_blocking(move || red_hare.set_string(k, v))
        .await
        .map_err(|e| e.to_string())?;

    ret.map_err(|e| e.to_string())
}

pub async fn set_string_with_expire(
    k: String,
    v: String,
    expire_time: u128,
) -> Result<bool, String> {
    let red_hare = RedHare::singleton();
    let ret = spawn_blocking(move || red_hare.set_string_with_expire(k, v, expire_time))
        .await
        .map_err(|e| e.to_string())?;

    ret.map_err(|e| e.to_string())
}



pub async  fn get_string(k: String) -> Result<Option<String>, String> {
    let red_hare = RedHare::singleton();
    let ret = spawn_blocking(move || red_hare.get_string(k))
        .await
        .map_err(|e| e.to_string())?;

    ret.map_err(|e| e.to_string())
}
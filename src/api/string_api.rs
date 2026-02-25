use crate::protocol::red_hare_wrapper;
use axum::Json;
use axum::extract::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StringSaveRequest {
    pub key: String,
    pub value: String,
    pub expire_time: Option<u128>,
}

pub async fn get_string(Path(key): Path<String>) -> Result<Json<String>, String> {
    let ret = red_hare_wrapper::get_string(key).await?;
    match ret {
        None => Ok(Json(String::from(""))),
        Some(ret) => Ok(Json(ret)),
    }
}
pub async fn set_string(Json(payload): Json<StringSaveRequest>) -> Result<Json<bool>, String> {
    let ret;
    match payload.expire_time {
        Some(expire_time) => {
            ret = red_hare_wrapper::set_string_with_expire(payload.key, payload.value, expire_time).await;
        }
        None => {
            ret = red_hare_wrapper::set_string(&payload.key, payload.value).await;
        }
    }
    match ret {
        Ok(ret) => Ok(Json(ret)),
        Err(error) => Err(error),
    }
}

use crate::protocol::red_hare_wrapper;
use axum::Json;
use axum::extract::Path;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;

#[derive(Deserialize, Serialize)]
pub struct StringSaveRequest {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub key: String,
    pub value: String,

    #[serde(default = "default_expire_time")]
    pub expire_time: u128,
}
fn default_expire_time() -> u128 {
    0
}

fn deserialize_non_empty_string<'a, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'a>,
{
    // 先正常反序列化成 String
    let ret = String::deserialize(deserializer)?;

    // 验证是否为空
    if ret.is_empty() {
        // 如果为空，返回错误（使用 serde 的错误类型）
        return Err(Error::custom("key cannot be empty"));
    }

    // 验证通过，返回字符串
    Ok(ret)
}
pub async fn get_string(Path(key): Path<String>) -> Result<Json<String>, String> {
    let ret = red_hare_wrapper::get_string(key).await?;
    match ret {
        None => Ok(Json(String::from(""))),
        Some(ret) => Ok(Json(ret)),
    }
}
pub async fn set_string(Json(payload): Json<StringSaveRequest>) -> Result<Json<bool>, String> {
    let ret =
        red_hare_wrapper::set_string_with_expire(payload.key, payload.value, payload.expire_time)
            .await;
    match ret {
        Ok(ret) => Ok(Json(ret)),
        Err(error) => Err(error),
    }
}

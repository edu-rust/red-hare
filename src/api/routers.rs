use crate::protocol::red_hare_wrapper;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/kv/save", post(set_string))
        .route("/api/key/{key}", get(get_string))
}

#[derive(Deserialize, Serialize)]
struct StringSaveRequest {
    key: String,
    value: String,
    expire_time: Option<u128>,
}

async fn get_string(Path(key): Path<String>) -> Result<Json<String>, String> {
    let ret = red_hare_wrapper::get_string(key).await?;
    match ret {
        None => Ok(Json(String::from(""))),
        Some(ret) => Ok(Json(ret)),
    }
}
async fn set_string(Json(payload): Json<StringSaveRequest>) -> Result<Json<bool>, String> {
    let ret;
    match payload.expire_time {
        Some(expire_time) => {
            ret = red_hare_wrapper::set_string_with_expire(payload.key, payload.value, expire_time)
                .await;
        }
        None => {
            ret = red_hare_wrapper::set_string(payload.key, payload.value).await;
        }
    }
    match ret {
        Ok(ret) => Ok(Json(ret)),
        Err(error) => Err(error),
    }
}

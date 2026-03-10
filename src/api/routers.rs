use crate::api::string_api::{get_string, set_string};
use axum::Router;
use axum::routing::{get, post};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/kv/save", post(set_string))
        .route("/api/key/{key}", get(get_string))
        .route("/health", get(|| async { "OK" }))
}

use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use super::State;

pub fn router() -> Router<State> {
    Router::new()
        .route("/ping", get(ping))
        //.nest("/api", router())
}

async fn ping() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "msg": "Server left!",
    }))
}

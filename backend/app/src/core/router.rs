use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use axum::http::{StatusCode, Uri};
use serde::Serialize;

use super::{ApiError, ApiResultJson, State};
use crate::domains::domains_router;

pub fn router() -> Router<State> {
    Router::new()
        .route("/ping", get(ping))
        .nest("/api", domains_router())
        .fallback(fallback)
}

#[derive(Serialize)]
struct Fallback {
    path: String,
    msg: String,
}

#[derive(Serialize)]
struct Ping {
    msg: String,
}

async fn fallback(uri: Uri) -> ApiResultJson<Fallback> {
    let body: Fallback = Fallback {
        path: format!("{uri}").to_string(),
        msg: "Das ist kein gültiger Pfad!".to_string()
    };
    Ok((StatusCode::NOT_FOUND, Json(body)))
}

async fn ping() -> ApiResultJson<Ping> {
    let body: Ping = Ping {
        msg: "Server leuft!".to_string()
    };
    Ok((StatusCode::OK, Json(body)))
}

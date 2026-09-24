use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tower::ServiceExt;

use crate::core::AppState;

async fn pool() -> SqlitePool {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE notes (
            id TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'notes',
            created_at TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

fn router(pool: SqlitePool) -> Router {
    crate::core::router().with_state(AppState::new(pool))
}

pub async fn app() -> Router {
    router(pool().await)
}

pub async fn app_with_note(id: &str, title: &str, content: &str) -> Router {
    let pool = pool().await;

    sqlx::query("INSERT INTO notes (id, title, content, created_at) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(title)
        .bind(content)
        .bind("2026-09-23 12:00:00")
        .execute(&pool)
        .await
        .unwrap();

    router(pool)
}

pub async fn send_json(
    app: Router,
    method: &str,
    uri: &str,
    payload: Value,
) -> (StatusCode, Value) {
    let request: Request<Body> = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    (status, json)
}

pub async fn send_empty(app: Router, method: &str, uri: &str) -> (StatusCode, Value) {
    let request: Request<Body> = Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    (status, json)
}

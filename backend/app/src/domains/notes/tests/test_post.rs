use axum::http::StatusCode;
use serde_json::json;

use super::common::{app, send_json};

#[tokio::test]
async fn test_post_note_with_title_and_content() {
    let payload = json!({"title": "New note", "content": "Text"});

    let (status, body) = send_json(app().await, "POST", "/api/notes", payload).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["title"], "New note");
    assert_eq!(body["content"], "Text");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_post_note_without_content_uses_empty_string() {
    let payload = json!({"title": "Note without content"});

    let (status, body) = send_json(app().await, "POST", "/api/notes", payload).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["content"], "");
}

#[tokio::test]
async fn test_post_note_rejects_missing_title() {
    let payload = json!({"content": "Text"});

    let (status, _) = send_json(app().await, "POST", "/api/notes", payload).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_post_note_rejects_blank_title() {
    let payload = json!({"title": "   ", "content": "Text"});

    let (status, body) = send_json(app().await, "POST", "/api/notes", payload).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Titel darf nicht leer sein");
}

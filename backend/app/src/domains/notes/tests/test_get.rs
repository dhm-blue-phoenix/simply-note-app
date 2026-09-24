use axum::http::StatusCode;
use serde_json::json;

use super::common::{app, app_with_note, send_empty};

#[tokio::test]
async fn test_get_notes_returns_empty_array() {
    let (status, body) = send_empty(app().await, "GET", "/api/notes").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_get_notes_returns_notes() {
    let app = app_with_note(
        "11111111-1111-4111-8111-111111111111",
        "First note",
        "Note content",
    )
    .await;

    let (status, body) = send_empty(app, "GET", "/api/notes").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body[0]["title"], "First note");
    assert_eq!(body[0]["content"], "Note content");
}

#[tokio::test]
async fn test_get_note_returns_note() {
    let id = "22222222-2222-4222-8222-222222222222";
    let app = app_with_note(id, "A note", "Content").await;

    let (status, body) = send_empty(app, "GET", &format!("/api/notes/{id}")).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], id);
    assert_eq!(body["title"], "A note");
}

#[tokio::test]
async fn test_get_note_rejects_invalid_id() {
    let (status, body) = send_empty(app().await, "GET", "/api/notes/invalid").await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Ungültige ID");
}

#[tokio::test]
async fn test_get_note_returns_not_found_for_unknown_id() {
    let id = "33333333-3333-4333-8333-333333333333";

    let (status, body) = send_empty(app().await, "GET", &format!("/api/notes/{id}")).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "DB Eintrag nicht gefunden!");
}

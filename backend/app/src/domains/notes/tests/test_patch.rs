use axum::http::StatusCode;
use serde_json::json;

use super::common::{app, app_with_note, send_json};

const NOTE_ID: &str = "44444444-4444-4444-8444-444444444444";

#[tokio::test]
async fn test_patch_note_updates_title_only() {
    let payload = json!({"title": "Updated title"});

    let (status, body) = send_json(
        app_with_note(NOTE_ID, "Old title", "Keep this content").await,
        "PATCH",
        &format!("/api/notes/{NOTE_ID}"),
        payload,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["title"], "Updated title");
    assert_eq!(body["content"], "Keep this content");
}

#[tokio::test]
async fn test_patch_note_updates_content_only() {
    let payload = json!({"content": "Updated content"});

    let (status, body) = send_json(
        app_with_note(NOTE_ID, "Keep this title", "Old content").await,
        "PATCH",
        &format!("/api/notes/{NOTE_ID}"),
        payload,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["title"], "Keep this title");
    assert_eq!(body["content"], "Updated content");
}

#[tokio::test]
async fn test_patch_note_updates_title_and_content() {
    let payload = json!({"title": "New title", "content": "New content"});

    let (status, body) = send_json(
        app_with_note(NOTE_ID, "Old title", "Old content").await,
        "PATCH",
        &format!("/api/notes/{NOTE_ID}"),
        payload,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["title"], "New title");
    assert_eq!(body["content"], "New content");
}

#[tokio::test]
async fn test_patch_note_rejects_empty_patch() {
    let (status, body) = send_json(
        app_with_note(NOTE_ID, "Title", "Content").await,
        "PATCH",
        &format!("/api/notes/{NOTE_ID}"),
        json!({}),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Mindestens ein Feld muss angegeben werden");
}

#[tokio::test]
async fn test_patch_note_rejects_blank_title() {
    let (status, body) = send_json(
        app_with_note(NOTE_ID, "Title", "Content").await,
        "PATCH",
        &format!("/api/notes/{NOTE_ID}"),
        json!({"title": "   "}),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Titel darf nicht leer sein");
}

#[tokio::test]
async fn test_patch_note_rejects_invalid_id() {
    let (status, body) = send_json(
        app().await,
        "PATCH",
        "/api/notes/invalid",
        json!({"title": "Title"}),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Ungültige ID");
}

#[tokio::test]
async fn test_patch_note_returns_not_found_for_unknown_id() {
    let id = "55555555-5555-4555-8555-555555555555";

    let (status, body) = send_json(
        app().await,
        "PATCH",
        &format!("/api/notes/{id}"),
        json!({"title": "Title"}),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "DB Eintrag nicht gefunden!");
}

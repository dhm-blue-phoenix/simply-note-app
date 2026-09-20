use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;

use crate::core::{ApiResultJson, AppState as CoreState};
use super::super::models::{DtoNotes, NoteStatus, DtoNote, DtoNotePost, DtoNotePatch};
use super::super::service::{list_notes};

pub async fn get_notes(State(state): State<CoreState>) -> ApiResultJson<DtoNotes> {
    let notes: DtoNotes = list_notes(&state.db).await?;
    Ok((StatusCode::OK, Json(notes)))
}

pub async fn get_note() {}

pub async fn post_note() {}

pub async fn patch_note() {}

pub async fn delete_note() {}
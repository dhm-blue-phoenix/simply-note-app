use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use crate::core::ApiResultJson;
use super::super::models::{ DtoNotes, NoteStatus, DtoNote, DtoNotePost, DtoNotePatch };

pub async fn get_notes() -> ApiResultJson<DtoNotes> {
    let notes: DtoNotes = vec![];
    Ok((StatusCode::OK, Json(notes)))
}
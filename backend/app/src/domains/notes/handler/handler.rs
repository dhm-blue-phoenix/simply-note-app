use axum::extract::{Path, State};
use axum::Json;
use axum::http::StatusCode;
use sqlx::SqlitePool;
use crate::core::{ApiError, ApiResult, ApiResultJson, AppState as CoreState};
use crate::domains::notes::models::{DbNoteInsert, DbNoteUpdate};
use super::super::models::{DtoNotes, NoteStatus, DtoNote, DtoNotePost, DtoNotePatch};
use super::super::service::{list, find, add, edit, delete};

pub async fn get_notes(State(state): State<CoreState>) -> ApiResultJson<DtoNotes> {
    let pool: &SqlitePool = &state.db;

    let notes: DtoNotes = list(pool).await?;
    Ok((StatusCode::OK, Json(notes)))
}

pub async fn get_note(State(state): State<CoreState>, Path(id): Path<String>) -> ApiResultJson<DtoNote> {
    let pool: &SqlitePool = &state.db;
    let id: &str = id.trim();

    let note: DtoNote = find(pool, id).await?;
    Ok((StatusCode::OK, Json(note)))
}

pub async fn post_note(State(state): State<CoreState>, Json(data): Json<DtoNotePost>) -> ApiResultJson<DtoNote> {
    let pool: &SqlitePool = &state.db;
    let resp_data: DbNoteInsert = DbNoteInsert {
        title: data.title.trim().to_string(),
        content: data.content.trim().to_string(),
    };

    let note: DtoNote = add(pool, resp_data).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn patch_note(State(state): State<CoreState>, Path(id): Path<String>, Json(data): Json<DtoNotePatch>) -> ApiResultJson<DtoNote> {
    let pool: &SqlitePool = &state.db;
    let resp_data: DbNoteUpdate = DbNoteUpdate {
        id: id.trim().to_string(),
        title: match data.title {
            Some(title) => title.trim().to_string(),
            None => "".to_string()
        },
        content: match data.content {
            Some(content) => content.trim().to_string(),
            None => "".to_string()
        }
    };

    let note: DtoNote = edit(pool, resp_data).await?;
    Ok((StatusCode::OK, Json(note)))
}

pub async fn delete_note() {}
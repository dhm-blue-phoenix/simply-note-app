use sqlx::SqlitePool;

use crate::core::{ApiError, ApiResult};
use super::models::{DbNotes, DbNote, DtoNote, DtoNotes, NoteStatus, DtoNotePost, DbNoteInsert, DbNoteUpdate};
use super::db::{find_all, find_by_id, insert, update};

pub async fn list(pool: &SqlitePool) -> ApiResult<DtoNotes> {
    let db_data: DbNotes = find_all(pool).await?;

    if db_data.is_empty() {
        return Err(ApiError::NotFound("Keine Notitzen Gefunden!".into()));
    }

    let dto_data: DtoNotes = db_data.into_iter().map(|note| format_dto_model(note)).collect();
    Ok(dto_data)
}

pub async fn find(pool: &SqlitePool, id: &str) -> ApiResult<DtoNote> {
    if id.len() != 36 {
        return Err(ApiError::InvalidInput("Ungültige ID".to_string()));
    };

    let db_data: DbNote = find_by_id(pool, id).await?;
    let dto_data: DtoNote = format_dto_model(db_data);
    Ok(dto_data)
}

pub async fn add(pool: &SqlitePool, data: DbNoteInsert) -> ApiResult<DtoNote> {
    let db_data: DbNote = insert(pool, data).await?;
    let dto_data: DtoNote = format_dto_model(db_data);
    Ok(dto_data)
}

pub async fn edit(pool: &SqlitePool, data: DbNoteUpdate) -> ApiResult<DtoNote> {
    let db_data: DbNote = update(pool, data).await?;
    let dto_data: DtoNote = format_dto_model(db_data);
    Ok(dto_data)
}

pub async fn delete() {}

fn format_dto_model(db_data: DbNote) -> DtoNote {
    DtoNote {
        id: db_data.id.to_string(),
        title: db_data.title,
        content: db_data.content,
        status: db_data.status,
        created_at: db_data.created_at.to_string(),
    }
}
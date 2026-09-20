use sqlx::SqlitePool;

use crate::core::ApiResult;
use super::models::{DbNotes, DtoNote, DtoNotes};
use super::db::{find_all};

pub async fn list_notes(pool: &SqlitePool) -> ApiResult<DtoNotes> {
    let db_data: DbNotes = find_all(pool).await?;
    let dto_data: DtoNotes = db_data.into_iter().map(|note| DtoNote {
        id: note.id.to_string(),
        title: note.title,
        content: note.content,
        status: note.status,
        created_at: note.created_at.to_string(),
    }).collect();
    Ok(dto_data)
}

pub async fn find_note() {}

pub async fn add_note() {}

pub async fn edit_note() {}

pub async fn delete_note() {}
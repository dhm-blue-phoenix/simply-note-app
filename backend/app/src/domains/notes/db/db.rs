//noinspection RsInternalFileMetadata
use super::super::models::{DbNotes, DbNote};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::core::ApiResult;

pub async fn find_all(db: &SqlitePool) -> ApiResult<DbNotes> {
    let sql: &str = "SELECT id, title, content, status, created_at FROM notes ORDER BY created_at";
    let data: DbNotes = sqlx::query_as::<_, DbNote>(sql).fetch_all(db).await?;
    Ok(data)
}
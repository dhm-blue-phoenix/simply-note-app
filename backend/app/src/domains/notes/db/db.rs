use sqlx::sqlite::SqliteQueryResult;
//noinspection RsInternalFileMetadata
use super::super::models::{DbNotes, DbNote, DbNoteInsert, DbNoteUpdate};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::core::ApiResult;

pub async fn find_all(pool: &SqlitePool) -> ApiResult<DbNotes> {
    let sql: &str = "SELECT id, title, content, status, created_at FROM notes ORDER BY created_at";
    let result: DbNotes = sqlx::query_as::<_, DbNote>(sql)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> ApiResult<Option<DbNote>> {
    let sql: &str = "SELECT id, title, content, status, created_at FROM notes WHERE id = ?";
    let result: Option<DbNote> = sqlx::query_as::<_, DbNote>(sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn insert(pool: &SqlitePool, data: DbNoteInsert) -> ApiResult<Option<DbNote>> {
    let date: String = chrono::Local::now().naive_local().to_string();
    let buf: [u8; 16] = *b"abcdefghijklmnop";
    let id: String = Uuid::new_v8(buf).to_string();

    let sql: &str = "INSERT INTO notes (id, title, content, created_at) VALUES (?,?,?,?)";
    let result: SqliteQueryResult = sqlx::query::<_>(sql)
        .bind(&id)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&date)
        .execute(pool)
        .await?;

    let notes: Option<DbNote> = find_by_id(pool, &id).await?;
    Ok(notes)
}

pub async fn update(pool: &SqlitePool, data: DbNoteUpdate) -> ApiResult<Option<DbNote>> {
    let sql: &str = "UPDATE notes SET title = ?, content = ? WHERE id = ?";
    sqlx::query(sql)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.id)
        .execute(pool)
        .await?;

    let notes: Option<DbNote> = find_by_id(pool, &data.id).await?;
    Ok(notes)
}

pub async fn delete(pool: &SqlitePool, id: &str) -> ApiResult<bool> {
    let sql: &str = "DELETE FROM notes WHERE id = ?";
    let result = sqlx::query(sql)
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() == 0)
}
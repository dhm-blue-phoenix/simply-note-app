use super::super::models::{DbNote, DbNoteInsert, DbNotes};
use crate::core::ApiResult;
use crate::domains::notes::models::DbNoteUpdate;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

pub async fn find_all(pool: &SqlitePool) -> ApiResult<DbNotes> {
    let sql: &str = "SELECT id, title, content, status, created_at FROM notes ORDER BY created_at";
    let result: DbNotes = sqlx::query_as::<_, DbNote>(sql)
        .fetch_all(pool)
        .await?;

    println!("{:#?}", result);

    Ok(result)
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> ApiResult<DbNote> {
    let sql: &str = "SELECT id, title, content, status, created_at FROM notes WHERE id = ?";
    let result: DbNote = sqlx::query_as::<_, DbNote>(sql)
        .bind(&id)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn insert(pool: &SqlitePool, data: DbNoteInsert) -> ApiResult<DbNote> {
    let date: String = chrono::Local::now().naive_local().to_string();
    let id: String = Uuid::new_v4().to_string();

    let sql: &str = "INSERT INTO notes (id, title, content, created_at) VALUES (?,?,?,?)\
                    RETURNING id, title, content, status, created_at";
    let result: DbNote = sqlx::query_as::<_, DbNote>(sql)
        .bind(&id)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&date)
        .fetch_one(pool)
        .await?;

    println!("TESTING: INSERT {:?}", result);

    Ok(result)
}

pub async fn update(pool: &SqlitePool, data: DbNoteUpdate) -> ApiResult<DbNote> {
    let sql: &str = "UPDATE notes SET title = COALESCE(?, title), content = COALESCE(?, content) WHERE id = ?\
                    RETURNING id, title, content, status, created_at";
    let result: DbNote = sqlx::query_as::<_, DbNote>(sql)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.id)
        .fetch_one(pool)
        .await?;

    println!("TESTING: UPDATE {:?}", result);

    Ok(result)
}

#[derive(FromRow, Debug)]
struct DB_RESULT_DELETE {
    id: Uuid,
}

pub async fn delete(pool: &SqlitePool, id: &str) -> ApiResult<bool> {
    let sql: &str = "DELETE FROM notes WHERE id = ? RETURNING id";
    let result: DB_RESULT_DELETE = sqlx::query_as::<_, DB_RESULT_DELETE>(sql)
        .bind(&id)
        .fetch_one(pool)
        .await?;

    println!("TESTING: DELETE {:?}", result);
    println!("TESTING: DELETE ID != DEFAULD {:?}", result.id != Uuid::default());

    Ok(result.id != Uuid::default())
}
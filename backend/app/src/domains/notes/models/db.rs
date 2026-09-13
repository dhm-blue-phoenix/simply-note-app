use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

use super::models::NoteStatus;

pub type DbNotes = Vec<DbNote>;

// DB Models
#[derive(FromRow)]
pub struct DbNote {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub status: NoteStatus,
    pub category_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
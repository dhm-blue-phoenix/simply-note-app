//noinspection ALL
use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NoteStatus {
    Notes,
    Trash,
}
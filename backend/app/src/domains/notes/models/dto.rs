use serde::{Serialize, Deserialize};

use super::models::NoteStatus;

pub type DtoNotes = Vec<DtoNote>;

// Response Model ( GET )
#[derive(Serialize)]
pub struct DtoNote {
    pub id: String,
    pub title: String,
    pub content: String,
    pub status: NoteStatus,
    pub created_at: String,
}

// Request Model ( POST, PATCH )
#[derive(Deserialize)]
pub struct DtoNotePost {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct DtoNotePatch {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<NoteStatus>,
}
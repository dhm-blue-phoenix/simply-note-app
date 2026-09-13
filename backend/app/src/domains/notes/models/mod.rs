mod dto;
mod db;
mod models;

pub use db::{ DbNotes, DbNote };
pub use dto::{ DtoNotes, DtoNote, DtoNotePost, DtoNotePatch };
pub use models::NoteStatus;
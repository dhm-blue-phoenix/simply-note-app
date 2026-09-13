use axum::{Router, routing::{get, post, patch, delete}};

use crate::core::AppState;
use super::handler::handler::get_notes;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notes", get(get_notes))
        //.route("notes/{id}")
}

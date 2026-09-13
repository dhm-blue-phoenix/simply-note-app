use axum::Router;

use crate::core::State;
use super::notes::notes_router;

pub fn router()-> Router<State> {
    Router::new()
        .merge(notes_router())
}
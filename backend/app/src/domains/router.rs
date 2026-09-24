use axum::Router;

use crate::core::AppState;
use super::notes::notes_router;

pub fn router() -> Router<AppState> {
    Router::new().merge(notes_router())
}
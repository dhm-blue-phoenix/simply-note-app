use sqlx::SqlitePool;

use super::init_pool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

pub async fn init_state(db_url: &str) -> AppState {
    init_pool(db_url).await.expect("Datenbankverbindung konnte nicht initialisiert werden")
}

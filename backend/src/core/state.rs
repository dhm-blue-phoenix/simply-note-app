use sqlx::SqlitePool;

use super::init_pool;

#[derive(Clone)]
pub struct State {
    pub db: SqlitePool,
}

impl State {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

pub async fn init_state(db_url: &str) -> State{
    init_pool(db_url).await.expect("Datenbankverbindung konnte nicht initialisiert werden")
}

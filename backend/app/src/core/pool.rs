use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::time::Duration;

use super::State;

pub async fn init_pool(db_url: &str) -> Result<State, sqlx::Error> {
    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(db_url)
        .await?;
    Ok(State::new(pool))
}
mod core;

use axum::Router;
use tokio::net::TcpListener;
use dotenvy::dotenv;

use crate::core::{init_state, Environment };

#[tokio::main]
async fn main() {
    dotenv().expect(".env Datei konnte nicht geladen werden!");
    let config: Environment = Environment::load();

    let app: Router = core::router().with_state(init_state(&config.db_url).await);

    let listener: TcpListener = tokio::net::TcpListener::bind(config.addr)
        .await
        .expect("Das binden der ip auf den TcpListener ist Fehlgeschlagen");

    println!("\n Server leuft auf http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Server konnte nicht gestartet werden");
}

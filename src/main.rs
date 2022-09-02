use std::net::TcpListener;

use sqlx::{PgPool, Connection};
use zero2prod::{startup::run, configuration::get_configuration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");
    let adress = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(adress)?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener, connection_pool)?.await
}

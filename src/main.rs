//! src/main.rs
use sqlx::PgPool;
use std::net::TcpListener;
use email_newsletter::startup::run;
use email_newsletter::configurations::get_configuration;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if cannot read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

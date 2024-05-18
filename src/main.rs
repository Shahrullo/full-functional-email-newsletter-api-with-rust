//! src/main.rs
use std::net::TcpListener;
use email_newsletter::startup::run;
use email_newsletter::configurations::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if cannot read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}

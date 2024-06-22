//! src/main.rs
use email_newsletter::startup::Application;
use email_newsletter::configurations::get_configuration;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // call the subscriber
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if cannot read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}

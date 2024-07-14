//! src/main.rs
use anyhow::Ok;
use email_newsletter::startup::Application;
use email_newsletter::configurations::get_configuration;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use email_newsletter::issue_delivery_worker::run_worker_until_stopped;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // call the subscriber
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if cannot read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    

    let application = Application::build(configuration.clone())
        .await?
        .run_until_stopped();
    let worker = run_worker_until_stopped(configurations);

    tokio::select! {
        _ = application => {},
        _ = worker => {},
    };

    Ok(())
}

//! src/main.rs
use tokio::task::JoinError;
use std::fmt::{Debug, Display};
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
        .await?;
    
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configurations));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    };

    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl Debug + Display>, JoinError>
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} falied",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}

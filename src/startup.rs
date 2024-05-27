use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::email_client::{self, EmailClient};
use crate::routes::{health_check, subscribe};

// We need to mark `run` as public.
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
        })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server)
}
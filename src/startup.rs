use crate::routes::{health_check, subscribe};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgConnection;

// We need to mark `run` as public.
pub fn run(
    listener: TcpListener,
    connection: PgConnection
) -> Result<Server, std::io::Error> {
    // wrap the connection in a smart pointer
    let connection = web::Data::new(connection);
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(connection.clone())
        })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server)
}
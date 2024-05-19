use actix_web::{web, HttpResponse};
use sqlx::PgConnection;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// We always return a 200 OK
pub async fn subscribe(
    _form: web::Form<FormData>,
    // retrieving a connection from the application state
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4();
        form.email,
        form.name,
        Utc::now()
    )
    // use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await();
    HttpResponse::Ok().finish()
}
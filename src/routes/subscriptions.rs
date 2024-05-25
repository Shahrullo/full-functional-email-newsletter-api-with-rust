use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::{naive, Utc};
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(_form, pool),
    fields (
        subscriber_email = %_form.email,
        subscriber_name = %_form.name,
    )
)]
// We always return a 200 OK
pub async fn subscribe(
    _form: web::Form<FormData>,
    // retrieving a connection from the application state
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let name = match SubscriberName::parse(_form.0.name) {
        Ok(name) => name,
        // return earily if the name is invalid, with a 400
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subscriber = NewSubscriber {
        email: _form.0.email,
        name,
    };
    
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,   
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subcsribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    // use `get_ref` to get an immutable reference to the `PgConnection`
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
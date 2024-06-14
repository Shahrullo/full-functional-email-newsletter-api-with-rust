use actix_web::web;
use secrecy::Secret;
use actix_web::HttpResponse;
use actix_web::http::header::LOCATION;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
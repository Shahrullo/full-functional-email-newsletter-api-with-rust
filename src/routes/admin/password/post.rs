use anyhow::Ok;
use secrecy::Secret;
use actix_web::{HttpResponse, web};
use sqlx::Type;

use crate::utils::{e500, see_other};
use crate::session_state::TypedSession;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };
    todo!()
}
use crate::session_state::TypedSession;
use crate::utils::{e_500, see_other};
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use std::fmt::Write;

#[derive(Template)]
#[template(path = "password.html")]
struct PasswordTemplate {
    error_message: String,
}

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e_500)?.is_none() {
        return Ok(see_other("/login"));
    }

    let mut error_message = String::new();

    for flash_message in flash_messages.iter() {
        writeln!(error_message, "{}", flash_message.content()).unwrap()
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(PasswordTemplate { error_message }.render().unwrap()))
}

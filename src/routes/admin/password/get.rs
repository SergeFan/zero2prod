use std::fmt::Write;

use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

use crate::authentication::UserId;

#[derive(Template)]
#[template(path = "password.html")]
struct PasswordTemplate {
    error_message: String,
}

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
    _user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_message = String::new();

    for flash_message in flash_messages.iter() {
        writeln!(error_message, "<p><i>{}</i></p>", flash_message.content()).unwrap()
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(PasswordTemplate { error_message }.render().unwrap()))
}

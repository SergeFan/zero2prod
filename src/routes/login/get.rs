use std::fmt::Write;

use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error_message: String,
}

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_message = String::new();

    for flash_message in flash_messages.iter() {
        writeln!(error_message, "{}", flash_message.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(LoginTemplate { error_message }.render().unwrap())
}

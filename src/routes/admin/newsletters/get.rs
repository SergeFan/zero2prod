use std::fmt::Write;

use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "newsletter.html")]
struct NewsletterTemplate {
    error_message: String,
    idempotency_key: String,
}

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_message = String::new();

    for flash_message in flash_messages.iter() {
        writeln!(error_message, "{}", flash_message.content()).unwrap();
    }

    let idempotency_key = Uuid::new_v4().to_string();

    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        NewsletterTemplate {
            error_message,
            idempotency_key,
        }
        .render()
        .unwrap(),
    ))
}

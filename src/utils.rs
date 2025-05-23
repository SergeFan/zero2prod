use std::fmt::{Debug, Display};

use actix_web::HttpResponse;
use actix_web::http::header::LOCATION;

pub fn e400<T>(e: T) -> actix_web::Error
where
    T: Debug + Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn e500<T>(e: T) -> actix_web::Error
where
    T: Debug + Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

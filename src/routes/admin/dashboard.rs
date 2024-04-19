use std::fmt::{Debug, Display};

use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

use crate::session_state::TypedSession;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    username: String,
}

fn e_500<T>(e: T) -> actix_web::Error
where
    T: Debug + Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub async fn admin_dashboard(
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e_500)? {
        get_username(user_id, &pool).await.map_err(e_500)?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(DashboardTemplate { username }.render().unwrap()))
}

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(r#"SELECT username FROM users WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await
        .context("Failed to perform a query to retrieve a username")?;

    Ok(row.username)
}

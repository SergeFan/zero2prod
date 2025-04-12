use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

use crate::authentication::UserId;
use crate::utils::e500;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    username: String,
}

pub async fn admin_dashboard(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(DashboardTemplate { username }.render().unwrap()))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(r#"SELECT username FROM users WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await
        .context("Failed to perform a query to retrieve a username")?;

    Ok(row.username)
}

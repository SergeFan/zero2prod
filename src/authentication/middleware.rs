use std::fmt::{Display, Formatter};
use std::ops::Deref;

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::middleware::Next;
use actix_web::{FromRequest, HttpMessage};
use uuid::Uuid;

use crate::session_state::TypedSession;
use crate::utils::{e_500, see_other};

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e_500)? {
        None => {
            let response = see_other("/login");

            Err(InternalError::from_response(
                anyhow::anyhow!("The user has not logged in"),
                response,
            )
            .into())
        }
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));

            next.call(req).await
        }
    }
}

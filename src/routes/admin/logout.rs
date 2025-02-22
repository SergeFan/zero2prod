use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;

use crate::authentication::UserId;
use crate::session_state::TypedSession;
use crate::utils::see_other;

pub async fn log_out(
    _user_id: web::ReqData<UserId>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    session.log_out();

    FlashMessage::info("You have successfully logged out").send();

    Ok(see_other("/login"))
}

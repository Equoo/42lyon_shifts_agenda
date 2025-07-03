use actix_session::Session;

use crate::{BackendResult, model::User};

pub fn require_user(session: &Session) -> BackendResult<User> {
    session
        .get::<User>("user")?
        .ok_or(crate::error::BackendError::Unauthorized)
}

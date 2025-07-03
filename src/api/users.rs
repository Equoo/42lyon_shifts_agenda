use actix_web::{Responder, get, web};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{BackendResult, db};

#[derive(Deserialize)]
struct UserRequest {
    username: String,
}

#[get("/users")]
pub async fn get_user(
    data: web::Json<UserRequest>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let UserRequest { username } = data.into_inner();
    let user = db::get_user(&db, &username).await?;
    Ok(web::Json(user))
}

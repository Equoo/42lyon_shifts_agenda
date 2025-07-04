use actix_web::{Responder, get, web};
use sqlx::MySqlPool;

use crate::{BackendResult, db};

#[get("/users/{username}")]
pub async fn get_user(
    username: web::Path<String>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let username = username.into_inner();
    let user = db::get_user(&db, &username).await?;
    Ok(web::Json(user))
}

use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    BackendResult,
    auth::{hash_password, verify_password},
    db,
    error::BackendError,
    model::User,
    util,
};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/auth/login")]
pub async fn login(
    data: web::Json<LoginRequest>,
    db: web::Data<MySqlPool>,
    session: Session,
) -> BackendResult<impl Responder> {
    let LoginRequest { username, password } = data.into_inner();
    let user = db::get_user(&db, &username).await?;
    match verify_password(&user, password) {
        Ok(_) => {}
        Err(argon2::password_hash::Error::Password) => return Err(BackendError::Unauthorized),
        Err(e) => return Err(BackendError::Argon2Error(e)),
    };
    session.insert("user", user)?;
    Ok(HttpResponse::Ok().finish())
}

#[post("/auth/register")]
pub async fn register(
    data: web::Json<LoginRequest>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let LoginRequest { username, password } = data.into_inner();
    let user = db::get_user(&db, &username).await;
    match user {
        Ok(_) => return Ok(HttpResponse::Conflict().finish()),
        Err(sqlx::Error::RowNotFound) => {}
        Err(e) => return Err(e.into()),
    };
    let password_hash = hash_password(password)?;
    let user = User {
        username,
        password_hash,
        grade: crate::model::UserGrade::Interested,
    };
    db::create_user(&db, user).await?;
    Ok(HttpResponse::Created().finish())
}

#[get("/auth/me")]
pub async fn me(session: Session) -> BackendResult<impl Responder> {
    let user = util::require_user(&session)?;
    Ok(HttpResponse::Ok().json(user))
}

use actix_session::Session;
use actix_web::{Responder, get, post, web};
use chrono::{Duration, NaiveDate};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{BackendResult, db, model::User, util};

#[derive(Deserialize)]
struct DateQuery {
    date: NaiveDate,
    slot: String, // "day" or "night"
}

#[derive(Deserialize)]
struct WeekQuery {
    start: NaiveDate,
}

/// Get all shifts + users for a full week
#[get("/shifts/week")]
pub async fn get_shift_week(
    query: web::Query<WeekQuery>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let start = query.start;
    let end = start + Duration::days(6);
    let shifts = db::get_shifts_with_users_between(&db, start, end).await?;
    Ok(web::Json(shifts))
}

/// Get users registered to a specific shift
#[get("/shifts/users")]
pub async fn get_shift_users(
    query: web::Query<DateQuery>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let DateQuery { date, slot } = query.into_inner();
    let users = db::get_users_for_shift(&db, date, &slot).await?;
    Ok(web::Json(users))
}

/// Register current user to a shift
#[get("/shifts/register")]
pub async fn register_to_shift(
    session: Session,
    db: web::Data<MySqlPool>,
    query: web::Query<DateQuery>,
) -> BackendResult<impl Responder> {
    let DateQuery { date, slot } = query.into_inner();
    if date >= chrono::Utc::now().date_naive() {
        let user: User = util::require_user(&session)?;
        db::add_user_to_shift(&db, date, &slot, &user.login).await?;
        let updated_shift = db::get_shift_with_users(&db, date, &slot).await?;
        Ok(web::Json(updated_shift))
    } else {
        Err(crate::error::BackendError::Unauthorized)
    }
}

/// Deregister current user from a shift
#[get("/shifts/deregister")]
pub async fn deregister_from_shift(
    session: Session,
    db: web::Data<MySqlPool>,
    query: web::Query<DateQuery>,
) -> BackendResult<impl Responder> {
    let DateQuery { date, slot } = query.into_inner();
    let user: User = util::require_user(&session)?;
    db::remove_user_from_shift(&db, date, &slot, &user.login).await?;
    let updated_shift = db::get_shift_with_users(&db, date, &slot).await?;
    Ok(web::Json(updated_shift))
}

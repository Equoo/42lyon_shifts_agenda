use actix_session::Session;
use actix_web::{Responder, get, post, web};
use chrono::{Duration, NaiveDate};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    BackendResult, db,
    model::{User, UserGrade},
    util,
};

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
#[post("/shifts/register")]
pub async fn register_to_shift(
    session: Session,
    db: web::Data<MySqlPool>,
    query: web::Json<DateQuery>,
) -> BackendResult<impl Responder> {
    let user = util::require_user(&session)?;
    let DateQuery { date, slot } = query.into_inner();
    let now = chrono::Utc::now().naive_local();
    let time = match slot.as_str() {
        "day" => chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        "night" => chrono::NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
        _ => return Err(crate::BackendError::InvalidSlot(slot)),
    };
    if now.date() > date || (date == now.date() && now.time() > time) {
        Err(crate::BackendError::Forbidden)
    } else {
        db::add_user_to_shift(&db, date, &slot, &user.login).await?;
        let updated_shift = db::get_shift_with_users(&db, date, &slot).await?;
        Ok(web::Json(updated_shift))
    }
}

#[post("/shift/register/{login}")]
pub async fn register_user_to_shift(
    session: Session,
    db: web::Data<MySqlPool>,
    query: web::Json<DateQuery>,
    login: web::Path<String>,
) -> BackendResult<impl Responder> {
    let user = util::require_user(&session)?;
    if user.grade != UserGrade::President && user.grade != UserGrade::Coordinator {
        return Err(crate::BackendError::Forbidden);
    }
    let DateQuery { date, slot } = query.into_inner();
    let login = login.into_inner();
    db::add_user_to_shift(&db, date, &slot, &login).await?;
    let updated_shift = db::get_shift_with_users(&db, date, &slot).await?;
    Ok(web::Json(updated_shift))
}

/// Deregister current user from a shift
#[post("/shifts/unregister")]
pub async fn unregister_from_shift(
    session: Session,
    db: web::Data<MySqlPool>,
    query: web::Json<DateQuery>,
) -> BackendResult<impl Responder> {
    let DateQuery { date, slot } = query.into_inner();
    let now = chrono::Utc::now().naive_local();
    let time = match slot.as_str() {
        "day" => chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        "night" => chrono::NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
        _ => return Err(crate::BackendError::InvalidSlot(slot)),
    };
    let user: User = util::require_user(&session)?;
    if now.date() > date || (date == now.date() && now.time() > time) {
        Err(crate::BackendError::Forbidden)
    } else {
        db::remove_user_from_shift(&db, date, &slot, &user.login).await?;
        let updated_shift = db::get_shift_with_users(&db, date, &slot).await?;
        Ok(web::Json(updated_shift))
    }
}

#[post("/shift/unregister/{login}")]
pub async fn unregister_user_from_shift(
    session: Session,
    db: web::Data<MySqlPool>,
    query: web::Json<DateQuery>,
    login: web::Path<String>,
) -> BackendResult<impl Responder> {
    let user = util::require_user(&session)?;
    if user.grade != UserGrade::President && user.grade != UserGrade::Coordinator {
        return Err(crate::BackendError::Forbidden);
    }
    let DateQuery { date, slot } = query.into_inner();
    let login = login.into_inner();
    db::remove_user_from_shift(&db, date, &slot, &login).await?;
    let updated_shift = db::get_shift_with_users(&db, date, &slot).await?;
    Ok(web::Json(updated_shift))
}

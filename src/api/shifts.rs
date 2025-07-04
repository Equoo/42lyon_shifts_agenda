use actix_web::{Responder, get, web};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{BackendResult, db};

#[derive(Deserialize)]
struct DateRangeRequest {
    start: NaiveDate,
    end: NaiveDate,
}

#[get("/shifts/{id}")]
pub async fn get_shift(
    id: web::Path<i32>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let shift_id = id.into_inner();
    let shift = db::get_shift(&db, shift_id).await?;
    Ok(web::Json(shift))
}

#[get("/shifts/{id}/users")]
pub async fn get_shift_users(
    id: web::Path<i32>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let shift_id = id.into_inner();
    let shift_users = db::get_users_for_shift(&db, shift_id).await?;
    Ok(web::Json(shift_users))
}

#[get("/shifts/range")]
pub async fn get_shift_range(
    dates: web::Query<DateRangeRequest>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let DateRangeRequest { start, end } = dates.into_inner();
    let shifts = db::get_shifts_between_dates(&db, start, end).await?;
    Ok(web::Json(shifts))
}

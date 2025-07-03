use actix_web::{Responder, get, web};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{BackendResult, db};

#[derive(Deserialize)]
struct ShiftRequest {
    shift_id: i32,
}

#[derive(Deserialize)]
struct DateRangeRequest {
    start: NaiveDate,
    end: NaiveDate,
}

#[get("/shifts")]
pub async fn get_shift(
    data: web::Json<ShiftRequest>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let ShiftRequest { shift_id } = data.into_inner();
    let shift = db::get_shift(&db, shift_id).await?;
    Ok(web::Json(shift))
}

#[get("/shifts/users")]
pub async fn get_shift_users(
    data: web::Json<ShiftRequest>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let ShiftRequest { shift_id } = data.into_inner();
    let shift_users = db::get_users_for_shift(&db, shift_id).await?;
    Ok(web::Json(shift_users))
}

#[get("/shifts/range")]
pub async fn get_shift_range(
    data: web::Json<DateRangeRequest>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let DateRangeRequest { start, end } = data.into_inner();
    let shifts = db::get_shifts_between_dates(&db, start, end).await?;
    Ok(web::Json(shifts))
}

use std::fmt::Write;

use actix_web::{HttpResponse, Responder, get, http::header::ContentType, web};
use sqlx::MySqlPool;

use crate::{BackendResult, db};

pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user)
        .service(get_shifts)
        .service(get_shift)
        .service(get_shift_users);
}

#[get("/users")]
async fn get_users(db: web::Data<MySqlPool>) -> BackendResult<impl Responder> {
    let users = db::get_users(&db).await?;
    let mut str = String::new();
    writeln!(str, "<ul>")?;
    for user in users {
        writeln!(
            str,
            "<li>Got user {} {} with grade {:?}</li>",
            user.id, user.username, user.grade
        )?;
    }
    writeln!(str, "</ul>")?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(str))
}

#[get("/user/{id}")]
async fn get_user(id: web::Path<i32>, db: web::Data<MySqlPool>) -> BackendResult<impl Responder> {
    let user = db::get_user(&db, id.into_inner()).await?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            "Got user {} {} with grade {:?}",
            user.id, user.username, user.grade
        )))
}

#[get("/shifts")]
async fn get_shifts(db: web::Data<MySqlPool>) -> BackendResult<impl Responder> {
    let shifts = db::get_shifts(&db).await?;
    let mut str = String::new();
    writeln!(str, "<ul>")?;
    for shift in shifts {
        writeln!(
            str,
            "<li>Got shift {} at date {}</li>",
            shift.id, shift.date
        )?;
    }
    writeln!(str, "</ul>")?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(str))
}

#[get("/shift/{id}")]
async fn get_shift(id: web::Path<i32>, db: web::Data<MySqlPool>) -> BackendResult<impl Responder> {
    let shift = db::get_shift(&db, id.into_inner()).await?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!("Got shift {} at date {}", shift.id, shift.date)))
}

#[get("/shift/{id}/users")]
async fn get_shift_users(
    id: web::Path<i32>,
    db: web::Data<MySqlPool>,
) -> BackendResult<impl Responder> {
    let shift = db::get_shift(&db, id.into_inner()).await?;
    let users = db::get_users_for_shift(&db, shift.id).await?;
    let mut str = String::new();
    writeln!(
        str,
        "Got shift {} at date {} with users: <ul>",
        shift.id, shift.date
    )?;
    for user in users {
        writeln!(
            str,
            "<li>{} {} ({:?})</li>",
            user.id, user.username, user.grade
        )?;
    }
    writeln!(str, "</ul>")?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(str))
}

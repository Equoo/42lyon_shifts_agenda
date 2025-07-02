use std::env;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::header::ContentType,
    middleware, post,
    web::{self, Data},
};
use error::BackendError;
use serde::Deserialize;
use sqlx::SqlitePool;

mod db;
mod error;

type BackendResult<T> = Result<T, BackendError>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("hell word")
}

#[get("/{msg_id}")]
async fn get_msg(
    msg_id: web::Path<i64>,
    db: web::Data<SqlitePool>,
) -> BackendResult<impl Responder> {
    let msg = db::get_msg(&db, msg_id.into_inner()).await?;
    Ok(HttpResponse::Ok().body(format!("This message: {}", msg)))
}

#[derive(Deserialize)]
struct Message {
    message: String,
}

#[post("/post")]
async fn post_msg(
    msg: web::Json<Message>,
    db: web::Data<SqlitePool>,
) -> BackendResult<impl Responder> {
    let id = db::create_msg(&db, &msg.message).await?;
    Ok(HttpResponse::Created().body(format!("{}", id)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let db = SqlitePool::connect("database.db")
        .await
        .expect("failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(get_msg)
            .service(post_msg)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

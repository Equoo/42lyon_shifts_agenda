use std::env;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::header::ContentType,
    middleware, post,
    web::{self, Data},
};
use serde::Deserialize;
use sqlx::SqlitePool;

// TODO: error management!!!
async fn get_message(db: &SqlitePool, id: i64) -> String {
    sqlx::query!(
        "SELECT message FROM messages
        WHERE id = ?",
        id
    )
    .fetch_one(db)
    .await
    .expect("wawa")
    .message
}

// TODO: error management!!!
async fn post_message(db: &SqlitePool, msg: &str) -> i64 {
    sqlx::query!("INSERT INTO messages (id, message) VALUES (NULL, ?)", msg)
        .execute(db)
        .await
        .expect("wawa")
        .last_insert_rowid()
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("hell word")
}

#[get("/{msg_id}")]
async fn get_msg(msg_id: web::Path<i64>, db: web::Data<SqlitePool>) -> impl Responder {
    let msg = get_message(&db, msg_id.into_inner()).await;
    HttpResponse::Ok().body(format!("This message: {}", msg))
}

#[derive(Deserialize)]
struct Message {
    message: String,
}

#[post("/post")]
async fn post_msg(msg: web::Json<Message>, db: web::Data<SqlitePool>) -> impl Responder {
    let id = post_message(&db, &msg.message).await;
    HttpResponse::Created().body(format!("{}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let db = SqlitePool::connect("database.db").await.expect("wawa");
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

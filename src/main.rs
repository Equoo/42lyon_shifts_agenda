use std::env;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, http::header::ContentType, middleware, web::Data,
};
use error::BackendError;
use sqlx::MySqlPool;

mod db;
mod error;
mod model;
mod routes;

type BackendResult<T> = Result<T, BackendError>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("hell word")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();
    #[cfg(debug_assertions)]
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL variable not set");
    let db = MySqlPool::connect(&db_url)
        .await
        .expect("failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .configure(routes::configure_urls)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

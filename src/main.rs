use std::{env, fs};

use actix_files::Files;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::header::ContentType,
    middleware,
    web::{self, Data},
};
use error::BackendError;
use sqlx::MySqlPool;

mod api;
mod db;
mod error;
mod model;

type BackendResult<T> = Result<T, BackendError>;

async fn not_found_handler() -> BackendResult<HttpResponse> {
    Err(BackendError::NotFound)
}

#[get("/")]
async fn index() -> BackendResult<impl Responder> {
    let html = fs::read_to_string("index.html")?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
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
            .service(Files::new("/static", "resources").show_files_listing())
            .service(index)
            .configure(api::configure_endpoints)
            .default_service(web::to(not_found_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

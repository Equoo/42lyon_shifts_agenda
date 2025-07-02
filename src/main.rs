use std::env;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::header::ContentType,
    middleware,
    web::{self, Data},
};
use error::BackendError;
use sqlx::MySqlPool;
use tera::Tera;

mod db;
mod error;
mod model;
mod routes;

type BackendResult<T> = Result<T, BackendError>;

#[get("/")]
async fn hello(tera: web::Data<Tera>) -> BackendResult<impl Responder> {
    let html = tera.render("pages/index.html", &tera::Context::default())?;
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

    let tera = Tera::new("templates/**/*.html").expect("failed to initialize templating engine");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL variable not set");
    let db = MySqlPool::connect(&db_url)
        .await
        .expect("failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(tera.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .configure(routes::configure_urls)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

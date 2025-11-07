use std::{env, fs};

use actix_cors::Cors;
use actix_files::Files;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    cookie::Key,
    get,
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
mod util;

type BackendResult<T> = Result<T, BackendError>;

async fn not_found_handler() -> BackendResult<HttpResponse> {
    Err(BackendError::NotFound)
}

#[get("/")]
async fn index() -> BackendResult<impl Responder> {
    let html = fs::read_to_string("./frontend/build/index.html")?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

fn check_missing_env_vars() -> Vec<String> {
    const VARS: &[&str] = &[
        "DATABASE_URL",
        "FT_CLIENT_ID",
        "FT_CLIENT_SECRET",
        "FT_REDIRECT_URI",
    ];
    let mut missing = vec![];
    for var in VARS {
        if let Err(env::VarError::NotPresent) = env::var(var) {
            missing.push(var.to_string());
        }
    }
    missing
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();
    let vars = check_missing_env_vars();
    if !vars.is_empty() {
        panic!("environment variables not set: {vars:?}");
    }
    #[cfg(debug_assertions)]
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL variable not set");
    let db = MySqlPool::connect(&db_url)
        .await
        .expect("failed to connect to database");

    let key = Key::generate();

    println!("Starting server");
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allow_any_origin() // NOTE: To remove unsecure
        //     .allow_any_method()
        //     .allow_any_header()
        //     .supports_credentials();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    // TODO: REMOVE THIS IN PROD
                    .cookie_secure(false)
                    .build(),
            )
            // .wrap(cors)
            .app_data(Data::new(db.clone()))
            .service(Files::new("/static", "./frontend/build/static"))
            .service(index)
            .configure(api::configure_endpoints)
            .default_service(web::to(not_found_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

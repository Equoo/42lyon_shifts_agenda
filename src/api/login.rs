use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use sqlx::MySqlPool;
use std::env;

use crate::{
    BackendResult, db,
    model::{User, UserGrade},
    util,
};

#[post("/auth/42/login")]
pub async fn login_42() -> BackendResult<impl Responder> {
    let client_id = env::var("FT_CLIENT_ID").expect("missing FT_CLIENT_ID");
    let redirect_uri = env::var("FT_REDIRECT_URI").expect("missing FT_REDIRECT_URI");
    let auth_url = format!(
        "https://api.intra.42.fr/oauth/authorize?client_id={}&redirect_uri={}&response_type=code",
        client_id, redirect_uri
    );
    Ok(web::Json(auth_url))
}

#[post("/auth/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

#[post("/auth/42/callback")]
pub async fn callback_42(
    query: web::Json<CallbackQuery>,
    db: web::Data<MySqlPool>,
    session: Session,
) -> BackendResult<impl Responder> {
    let code = &query.code;
    let client_id = env::var("FT_CLIENT_ID").unwrap();
    let client_secret = env::var("FT_CLIENT_SECRET").unwrap();
    let redirect_uri = env::var("FT_REDIRECT_URI").unwrap();

    // Exchange code for access_token
    let token_resp: Value = Client::new()
        .post("https://api.intra.42.fr/oauth/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await?
        .json()
        .await?;

    let access_token = token_resp["access_token"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("no access token"))?;

    // Get user info
    let user_info: Value = Client::new()
        .get("https://api.intra.42.fr/v2/me")
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await?;

    let login = user_info["login"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("no login"))?
        .to_string();
    let img_url = user_info["image"]["link"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    // Upsert user in DB
    db::upsert_user(&db, &login, &img_url).await?;

    // Set session cookie
    session.insert(
        "user",
        User {
            login,
            img_url,
            grade: UserGrade::Interested,
        },
    )?;
    Ok(web::Json("Connected"))
}

#[get("/auth/me")]
pub async fn me(session: Session, db: web::Data<MySqlPool>) -> BackendResult<impl Responder> {
    let user = util::require_user(&session)?;
    let user = db::get_user(&db, &user.login).await?;
    Ok(HttpResponse::Ok().json(user))
}

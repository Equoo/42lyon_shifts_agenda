use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Resource not found")]
    NotFound,

    #[error("Database error")]
    SqlxError(sqlx::Error),

    #[error("Internal server error")]
    FormatError(std::fmt::Error),

    #[error("Internal server error")]
    IoError(std::io::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal server error")]
    Argon2Error(argon2::password_hash::Error),

    #[error("Cookie error")]
    CookieInsertError(actix_session::SessionInsertError),

    #[error("Cookie error")]
    CookieGetError(actix_session::SessionGetError),
}

impl ResponseError for BackendError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        use BackendError::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
            Unauthorized => StatusCode::UNAUTHORIZED,
            Forbidden => StatusCode::FORBIDDEN,
            CookieGetError(e) => e.status_code(),
            CookieInsertError(e) => e.status_code(),
            SqlxError(_) | FormatError(_) | IoError(_) | Argon2Error(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<sqlx::Error> for BackendError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::SqlxError(value),
        }
    }
}

impl From<std::fmt::Error> for BackendError {
    fn from(value: std::fmt::Error) -> Self {
        Self::FormatError(value)
    }
}

impl From<std::io::Error> for BackendError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<actix_session::SessionGetError> for BackendError {
    fn from(value: actix_session::SessionGetError) -> Self {
        Self::CookieGetError(value)
    }
}

impl From<actix_session::SessionInsertError> for BackendError {
    fn from(value: actix_session::SessionInsertError) -> Self {
        Self::CookieInsertError(value)
    }
}

impl From<argon2::password_hash::Error> for BackendError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Argon2Error(value)
    }
}

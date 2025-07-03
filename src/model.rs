use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    #[sqlx(try_from = "i32")]
    pub grade: UserGrade,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserGrade {
    Unknown,
    Interested,
    Novice,
    Partner,
    Bartender,
    President,
}

impl From<i32> for UserGrade {
    fn from(value: i32) -> Self {
        use UserGrade::*;
        match value {
            0 => Interested,
            1 => Novice,
            2 => Partner,
            3 => Bartender,
            4 => President,
            _ => Unknown,
        }
    }
}

impl From<UserGrade> for i32 {
    fn from(value: UserGrade) -> Self {
        use UserGrade::*;
        match value {
            Interested => 0,
            Novice => 1,
            Partner => 2,
            Bartender => 3,
            President => 4,
            Unknown => -1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Shift {
    pub id: i32,
    pub date: chrono::NaiveDate,
}

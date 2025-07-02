use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    #[sqlx(try_from = "i32")]
    pub grade: UserGrade,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Shift {
    pub id: i32,
    pub date: chrono::NaiveDate,
}

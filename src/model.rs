use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub img_url: String,
    #[sqlx(try_from = "i32")]
    pub grade: UserGrade,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UserGrade {
    Unknown,
    Interested,
    Novice,
    Member,
    Bartender,
    Coordinator,
    HonoraryMember,
    President,
}

impl From<i32> for UserGrade {
    fn from(value: i32) -> Self {
        use UserGrade::*;
        match value {
            0 => Interested,
            1 => Novice,
            2 => Member,
            3 => Bartender,
            4 => Coordinator,
            5 => HonoraryMember,
            6 => President,
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
            Member => 2,
            Bartender => 3,
            Coordinator => 4,
            HonoraryMember => 5,
            President => 6,
            Unknown => -1,
        }
    }
}

#[derive(Serialize)]
pub struct Shift {
    pub date: NaiveDate,
    pub slot: String, // "day" or "night"
    pub min_users: i32,
    pub users: Vec<User>,
}

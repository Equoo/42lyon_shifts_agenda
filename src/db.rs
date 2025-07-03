use chrono::NaiveDate;
use sqlx::MySqlPool;

use crate::model::{Shift, User};

pub async fn get_user(db: &MySqlPool, username: &str) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        "SELECT
            u.username,
            u.password_hash,
            u.grade
        FROM
            users u
        WHERE
            u.username = ?",
        username,
    )
    .fetch_one(db)
    .await
}

pub async fn create_user(db: &MySqlPool, user: User) -> sqlx::Result<()> {
    let User {
        username,
        password_hash,
        grade,
    } = user;
    let grade: i32 = grade.into();
    sqlx::query!(
        "INSERT INTO
            users (username, password_hash, grade)
        VALUES
            (?, ?, ?)",
        username,
        password_hash,
        grade,
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_shift(db: &MySqlPool, id: i32) -> sqlx::Result<Shift> {
    sqlx::query_as!(
        Shift,
        "SELECT
            s.id,
            s.shift_date as date
        FROM
            shifts s
        WHERE
            id = ?",
        id,
    )
    .fetch_one(db)
    .await
}

pub async fn get_users_for_shift(db: &MySqlPool, id: i32) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        "SELECT
            u.username,
            u.password_hash,
            u.grade
        FROM
            users u
        JOIN
            shift_user su ON u.username = su.username
        WHERE
            su.shift_id = ?",
        id,
    )
    .fetch_all(db)
    .await
}

pub async fn get_shifts_between_dates(
    db: &MySqlPool,
    start: NaiveDate,
    end: NaiveDate,
) -> sqlx::Result<Vec<Shift>> {
    sqlx::query_as!(
        Shift,
        "SELECT
            s.id,
            s.shift_date as date
        FROM
            shifts s
        WHERE
            (s.shift_date BETWEEN ? AND ?)",
        start,
        end,
    )
    .fetch_all(db)
    .await
}

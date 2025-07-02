use sqlx::MySqlPool;

use crate::model::{Shift, User};

pub async fn get_user(db: &MySqlPool, id: i32) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        "SELECT
            u.id,
            u.username,
            u.password_hash,
            u.grade
        FROM
            users u
        WHERE
            id = ?",
        id,
    )
    .fetch_one(db)
    .await
}

pub async fn get_users(db: &MySqlPool) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        "SELECT
            u.id,
            u.username,
            u.password_hash,
            u.grade
        FROM
            users u",
    )
    .fetch_all(db)
    .await
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

pub async fn get_shifts(db: &MySqlPool) -> sqlx::Result<Vec<Shift>> {
    sqlx::query_as!(
        Shift,
        "SELECT
            s.id,
            s.shift_date as date
        FROM
            shifts s",
    )
    .fetch_all(db)
    .await
}

pub async fn get_users_for_shift(db: &MySqlPool, id: i32) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        "SELECT
            u.id,
            u.username,
            u.password_hash,
            u.grade
        FROM
            users u
        JOIN
            shift_user su ON u.id = su.user_id
        WHERE
            su.shift_id = ?",
        id,
    )
    .fetch_all(db)
    .await
}

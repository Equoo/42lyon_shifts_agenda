use chrono::NaiveDate;
use sqlx::MySqlPool;

use crate::model::{Shift, User};

pub async fn upsert_user(pool: &MySqlPool, login: &str, image_url: &str) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO users (login, img_url, grade)
        VALUES (?, ?, ?)
        ON DUPLICATE KEY UPDATE login = VALUES(login), img_url = VALUES(img_url)
        "#,
        login,
        image_url,
        0
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user(db: &MySqlPool, login: &str) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        "SELECT
            u.login,
            u.img_url,
            u.grade
        FROM
            users u
        WHERE
            u.login = ?",
        login,
    )
    .fetch_one(db)
    .await
}

pub async fn get_shifts_with_users_between(
    pool: &MySqlPool,
    start: NaiveDate,
    end: NaiveDate,
) -> sqlx::Result<Vec<Shift>> {
    // Fetch all shifts between start and end, with their registered users
    let rows = sqlx::query!(
        r#"
        SELECT s.date_shift, s.slot, s.min_users,
               u.login, u.img_url, u.grade
        FROM shifts s
        LEFT JOIN shifts_user su
               ON s.date_shift = su.date_shift AND s.slot = su.slot
        LEFT JOIN users u ON u.login = su.login
        WHERE s.date_shift BETWEEN ? AND ?
        ORDER BY s.date_shift, s.slot
        "#,
        start,
        end
    )
    .fetch_all(pool)
    .await?;

    // group by (date_shift, slot)
    use std::collections::BTreeMap;
    let mut grouped: BTreeMap<(NaiveDate, String), Shift> = BTreeMap::new();
    for row in rows {
        let key = (row.date_shift, row.slot.clone());
        let entry = grouped.entry(key).or_insert(Shift {
            date: row.date_shift,
            slot: row.slot.clone(),
            min_users: row.min_users.unwrap_or(2),
            users: Vec::new(),
        });
        if let Some(login) = row.login {
            entry.users.push(User {
                login,
                img_url: row.img_url.unwrap_or_default(),
                grade: row.grade.unwrap_or(0).into(),
            });
        }
    }

    Ok(grouped.into_values().collect())
}

pub async fn get_shift_with_users(
    pool: &MySqlPool,
    date: NaiveDate,
    slot: &str,
) -> sqlx::Result<Shift> {
    let mut users = Vec::new();
    let rows = sqlx::query!(
        r#"
        SELECT u.login, u.img_url, u.grade, s.min_users
        FROM shifts s
        LEFT JOIN shifts_user su
               ON s.date_shift = su.date_shift AND s.slot = su.slot
        LEFT JOIN users u ON u.login = su.login
        WHERE s.date_shift = ? AND s.slot = ?
        "#,
        date,
        slot
    )
    .fetch_all(pool)
    .await?;

    let min_users = rows.first().and_then(|r| r.min_users).unwrap_or(2);

    for row in rows {
        if let Some(login) = row.login {
            users.push(User {
                login,
                img_url: row.img_url.unwrap_or_default(),
                grade: row.grade.unwrap_or(0).into(),
            });
        }
    }

    Ok(Shift {
        date,
        slot: slot.to_string(),
        min_users,
        users,
    })
}

pub async fn get_users_for_shift(
    pool: &MySqlPool,
    date: NaiveDate,
    slot: &str,
) -> sqlx::Result<Vec<User>> {
    let rows = sqlx::query!(
        r#"
        SELECT u.login, u.img_url, u.grade
        FROM shifts_user su
        JOIN users u ON u.login = su.login
        WHERE su.date_shift = ? AND su.slot = ?
        "#,
        date,
        slot
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| User {
            login: r.login,
            img_url: r.img_url,
            grade: r.grade.into(),
        })
        .collect())
}

pub async fn add_user_to_shift(
    pool: &MySqlPool,
    date: NaiveDate,
    slot: &str,
    login: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT IGNORE INTO shifts (date_shift, slot, min_users)
        VALUES (?, ?, 2)
        "#,
        date,
        slot
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT IGNORE INTO shifts_user (date_shift, slot, login)
        VALUES (?, ?, ?)
        "#,
        date,
        slot,
        login
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_user_from_shift(
    pool: &MySqlPool,
    date: NaiveDate,
    slot: &str,
    login: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM shifts_user
        WHERE date_shift = ? AND slot = ? AND login = ?
        "#,
        date,
        slot,
        login
    )
    .execute(pool)
    .await?;
    Ok(())
}

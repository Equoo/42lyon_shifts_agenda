use sqlx::SqlitePool;

pub async fn create_msg(db: &SqlitePool, msg: &str) -> sqlx::Result<i64> {
    let id = sqlx::query!("INSERT INTO messages (id, message) VALUES (NULL, ?)", msg)
        .execute(db)
        .await?
        .last_insert_rowid();
    Ok(id)
}

pub async fn get_msg(db: &SqlitePool, id: i64) -> sqlx::Result<String> {
    let rec = sqlx::query!("SELECT message FROM messages WHERE id = ?", id)
        .fetch_one(db)
        .await?;
    Ok(rec.message)
}

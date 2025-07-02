use sqlx::MySqlPool;

pub async fn create_msg(db: &MySqlPool, msg: &str) -> sqlx::Result<u64> {
    let id = sqlx::query!("INSERT INTO messages (id, message) VALUES (NULL, ?)", msg)
        .execute(db)
        .await?
        .last_insert_id();
    Ok(id)
}

pub async fn get_msg(db: &MySqlPool, id: u64) -> sqlx::Result<String> {
    let rec = sqlx::query!("SELECT message FROM messages WHERE id = ?", id)
        .fetch_one(db)
        .await?;
    Ok(rec.message)
}

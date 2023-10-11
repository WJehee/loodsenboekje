use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use leptos::*;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[server]
pub async fn create_user(user: UserCreate) -> Result<i64> {
    let db = mm.db();
    let id: i64 = sqlx::query!("INSERT INTO users (name) VALUES (?)", user.name) 
        .execute(db)
        .await
        .map_err(|_| Error::DataBaseError)?
        .last_insert_rowid();
    Ok(id)
}

#[server]
pub async fn get_user(id: i64) -> Result<User> {
    let db = mm.db();
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(db)
        .await
        .map_err(|_| Error::NotFound)?;
    Ok(result)
}

#[server]
pub async fn delete(id: i64) -> Result<()> {
    let db = mm.db();
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(())
}


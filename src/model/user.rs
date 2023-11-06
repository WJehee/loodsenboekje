use serde::{Deserialize, Serialize};
use leptos::*;
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::db;
    use sqlx::FromRow;
}}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[server]
pub async fn create_user(username: String) -> Result<i64, ServerFnError> {
    let db = db().await;
    let id: i64 = sqlx::query!("INSERT INTO users (name) VALUES (?)", username) 
        .execute(&db)
        .await?
        .last_insert_rowid();
    Ok(id)
}

#[server]
pub async fn get_user(id: i64) -> Result<User, ServerFnError> {
    let db = db().await;
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    Ok(result)
}

#[server]
pub async fn delete(id: i64) -> Result<(), ServerFnError> {
    let db = db().await;
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&db)
        .await?;
    Ok(())
}


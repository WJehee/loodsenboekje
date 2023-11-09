use serde::{Deserialize, Serialize};
use leptos::*;
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::db;
    use sqlx::FromRow;
}}

#[derive(Debug, Deserialize, Serialize)]
enum Permission {
    View,
    Delete,
    Add,
    // Update,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    // pub permissions: Vec<Permission>,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
struct SqlUser {
    id: i64,
    username: String,
    password: String,
}

#[server]
pub async fn create_user(username: String, password: String) -> Result<i64, ServerFnError> {
    // TODO: check for user creation password, defined in .env, this will dictate the user
    // permissions upon creation (either view, or all 3)
    let db = db().await;
    let id: i64 = sqlx::query!("INSERT INTO users (username, password) VALUES (?, ?)", username, password) 
        .execute(&db)
        .await?
        .last_insert_rowid();
    Ok(id)
}

#[server]
pub async fn get_user(id: i64) -> Result<User, ServerFnError> {
    let db = db().await;
    let sqluser = sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    let result = User {
        id: sqluser.id,
        name: sqluser.username,
        // TODO: set permission correctly
        // permissions: Vec::new(),
    };
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


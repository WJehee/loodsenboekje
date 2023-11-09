use serde::{Deserialize, Serialize};
use leptos::*;
use cfg_if::cfg_if;
use strum::{Display, EnumString};

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::db;
    use sqlx::FromRow;

    async fn get_permissions(user_id: i64) -> Option<Vec<SqlPermission>> {
        let db = db().await;
       sqlx::query_as!(SqlPermission, "SELECT permission FROM user_permissions WHERE user_id = ?", user_id)
           .fetch_all(&db)
           .await.ok()
    }
}}

#[derive(Debug, Display, Deserialize, Serialize, EnumString)]
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
    permissions: Vec<Permission>,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
struct SqlUser {
    id: i64,
    username: String,
    password: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
struct SqlPermission {
    permission: String,
}

#[server]
pub async fn create_user(username: String, password: String) -> Result<i64, ServerFnError> {
    use bcrypt::{hash, DEFAULT_COST};

    let db = db().await;
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let id: i64 = sqlx::query!("INSERT INTO users (username, password) VALUES (?, ?)", username, hashed_password) 
        .execute(&db)
        .await?
        .last_insert_rowid();

    // TODO: check for user creation password, defined in .env, this will dictate the user
    // permissions upon creation (either view, or all 3)
    let perm = Permission::View.to_string();
    sqlx::query!("INSERT INTO user_permissions (user_id, permission) VALUES (?, ?)", id, perm)
        .execute(&db)
        .await?;

    Ok(id)
}

#[server]
pub async fn get_user(id: i64) -> Result<User, ServerFnError> {
    // Needed for string to permission enum
    use std::str::FromStr;

    let db = db().await;
    let sqluser = sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    // TODO: Implement this as a function for the struct
    let result = User {
        id: sqluser.id,
        name: sqluser.username,
        permissions: get_permissions(sqluser.id)
            .await
            .unwrap()
            .iter()
            .map(|p| Permission::from_str(&p.permission).unwrap())
            .collect(), 
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


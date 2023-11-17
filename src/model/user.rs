use serde::{Deserialize, Serialize};
use leptos::*;
use cfg_if::cfg_if;
use std::fmt;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::db;
    use sqlx::FromRow;
    use crate::auth::user;

    #[derive(FromRow)]
    pub struct SqlUser {
        pub id: i64,
        pub username: String,
        pub password: String,
        pub is_writer: i64,
    }

    impl From<SqlUser> for User {
        fn from(sqluser: SqlUser) -> Self {
            User {
                id: sqluser.id,
                name: sqluser.username,
                // SQLite stores bool as int, 0 = false, 1 = true
                is_writer: sqluser.is_writer == 1,
            }
        }
    }

    pub async fn get_user_by_username(username: &str) -> Result<SqlUser, sqlx::Error> {
        let db = db().await;
        sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE username = ?", username)
            .fetch_one(&db)
            .await
    }
}}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    // TODO: maybe we do want an enum here in the end, reader, writer and admin
    pub is_writer: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

#[server(Register)]
pub async fn create_user(username: String, password: String, creation_password: String) -> Result<(), ServerFnError> {
    use bcrypt::{hash, DEFAULT_COST};
    use std::env;

    let read_password = env::var("READ_PASSWORD").unwrap();
    let write_password = env::var("WRITE_PASSWORD").unwrap();

    let is_writer = match creation_password {
        p if p == read_password => Ok(false),
        p if p == write_password => Ok(true),
        _ => {
            println!("Invalid account creation password");
            Err(ServerFnError::ServerError("Invalid account creation password".into()))
        },
    }?;

    let db = db().await;
    let username = username.to_ascii_lowercase();
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let id: i64 = sqlx::query!("INSERT INTO users (username, password, is_writer) VALUES (?, ?, ?)", username, hashed_password, is_writer)
        .execute(&db)
        .await?
        .last_insert_rowid();

    println!("Created user: '{username}', with id: '{id}'");
    leptos_axum::redirect("/login");
    Ok(())
}

#[server]
pub async fn get_user(id: i64) -> Result<User, ServerFnError> {
    user()?;
    let db = db().await;
    let sqluser = sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    Ok(sqluser.into())
}

#[server]
pub async fn delete(id: i64) -> Result<(), ServerFnError> {
    let user = user()?;
    match id {
        id if id == user.id => {
            // only allow deletion of own account if regular user
            let db = db().await;
            sqlx::query!("DELETE FROM users WHERE id = ?", id)
                .execute(&db)
                .await?;
            println!("{user}, deleted user with id: {id}");
            Ok(())
        },
        id => {
            // TODO: maybe add clause for admin user
            println!("{user} tried to delete account with id: {id}");
            Err(ServerFnError::ServerError("No permission to delete this account".into()))
        }
    }
}


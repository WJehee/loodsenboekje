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
        pub user_type: i64,
    }

    impl From<SqlUser> for User {
        fn from(sqluser: SqlUser) -> Self {
            User {
                id: sqluser.id,
                name: sqluser.username,
                user_type: match sqluser.user_type {
                    1 => UserType::WRITER,
                    2 => UserType::ADMIN,
                    // 0 or any other (should not happen) is set to lowest priviledges
                    _ => UserType::READER,
                },
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

const MIN_PASSWORD_LENGTH: usize = 8;

pub fn validate_password(passwd: &str) -> bool {
    passwd.len() >= MIN_PASSWORD_LENGTH
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UserType {
    READER,
    WRITER,
    ADMIN,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub user_type: UserType,
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

    let read_password = env::var("READ_PASSWORD").expect("READ_PASSWORD to be set");
    let write_password = env::var("WRITE_PASSWORD").expect("WRITE_PASSWORD to be set");
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD to be set");

    let user_type = match creation_password {
        p if p == read_password => Ok(UserType::READER),
        p if p == write_password => Ok(UserType::WRITER),
        p if p == admin_password => Ok(UserType::ADMIN),
        _ => {
            println!("Invalid account creation password");
            Err(ServerFnError::ServerError("Invalid account creation password".into()))
        },
    }?;

    if !validate_password(&password) {
        return Err(ServerFnError::ServerError(format!("Password is too short, requires at least {MIN_PASSWORD_LENGTH} characters")))
    }

    let db = db().await;
    let username = username.to_ascii_lowercase();
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user_type = user_type as i64;
    let id: i64 = sqlx::query!("INSERT INTO users (username, password, user_type) VALUES (?, ?, ?)", username, hashed_password, user_type)
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
    match (&user.user_type, id) {
        (UserType::ADMIN, id) |(_, id) if id == id => {
            let db = db().await;
            sqlx::query!("DELETE FROM users WHERE id = ?", id)
                .execute(&db)
                .await?;
            println!("{user}, deleted user with id: {id}");
            Ok(())
        },
        (_, id) => {
            println!("{user} tried to delete account with id: {id}");
            Err(ServerFnError::ServerError("No permission to delete this account".into()))
        }
    }
}


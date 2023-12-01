use serde::{Deserialize, Serialize};
use leptos::*;
use cfg_if::cfg_if;
use std::fmt;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::db;
    use sqlx::{FromRow, Transaction, Sqlite};
    use crate::auth::user;
    use crate::errors::Error;
    use log::info;

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
                    0 => UserType::Reader,
                    1 => UserType::Writer,
                    2 => UserType::Admin,
                    //  any other (should not happen) is set to lowest priviledges
                    _ => UserType::Inactive,
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

    pub async fn get_user_by_id(id: i64) -> Result<SqlUser, sqlx::Error> {
        let db = db().await;
        sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&db)
            .await
    }

    pub async fn get_user_by_id_tx(transaction: &mut Transaction<'_, Sqlite>, id: i64) -> Result<SqlUser, sqlx::Error> {
        sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(transaction.as_mut())
            .await
    }

    pub async fn create_inactive_user(transaction: &mut Transaction<'_, Sqlite>, username: &str) -> Result<i64, ServerFnError> {
        debug!("creating inactive user: {username}");
        let empty_password = String::new();
        let id: i64 = sqlx::query!("INSERT INTO users (username, password, user_type) VALUES (?, ?, ?)", username, empty_password, UserType::Inactive as i64)
            .execute(transaction.as_mut())
            .await?
            .last_insert_rowid();
        info!("Created inactive user: '{username}', with id: '{id}'");
        Ok(id)
    }

    pub fn prepare_username(username: &str) -> String {
        let mut u = username
            .trim()
            .to_ascii_lowercase();
        u.remove(0).to_uppercase().to_string() + &u
    }
}}

const MIN_PASSWORD_LENGTH: usize = 8;

pub fn validate_password(passwd: &str) -> bool {
    passwd.len() >= MIN_PASSWORD_LENGTH
}

pub fn validate_username(username: &str) -> bool {
    username.chars().all(char::is_alphabetic)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UserType {
    Reader,
    Writer,
    Admin,
    Inactive,
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
        p if p == read_password => UserType::Reader,
        p if p == write_password => UserType::Writer,
        p if p == admin_password => UserType::Admin,
        _ => {
            debug!("Invalid account creation password");
            return Err(Error::InvalidInput.into())
        },
    };

    if !validate_username(&username) {
        return Err(Error::InvalidInput.into())
    }
    if !validate_password(&password) {
        return Err(Error::InvalidInput.into())
    }

    let db = db().await;
    let username = prepare_username(&username); 
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user_type = user_type as i64;

    if let Ok(user) = get_user_by_username(&username).await {
        if user.user_type == UserType::Inactive as i64 {
            sqlx::query!("UPDATE users SET password = ?, user_type = ? WHERE id = ?", hashed_password, user_type, user.id)
                .execute(&db)
                .await?;
            info!("User: {username}, with id: {} activated their account", user.id);
        } else {
            debug!("User {username} already exists!");
            return Err(Error::InvalidInput.into())
        }
    } else {
        let id: i64 = sqlx::query!("INSERT INTO users (username, password, user_type) VALUES (?, ?, ?)", username, hashed_password, user_type)
            .execute(&db)
            .await?
            .last_insert_rowid();
        info!("Created user: '{username}', with id: '{id}'");
    }

    leptos_axum::redirect("/login");
    Ok(())
}

#[server]
pub async fn get_user(id: i64) -> Result<User, ServerFnError> {
    user()?;
    let sqluser = get_user_by_id(id).await?;
    Ok(sqluser.into())
}

#[server]
pub async fn delete(id: i64) -> Result<(), ServerFnError> {
    let user = user()?;
    match (&user.user_type, id) {
        (UserType::Admin, id) |(_, id) if id == id => {
            let db = db().await;
            sqlx::query!("DELETE FROM users WHERE id = ?", id)
                .execute(&db)
                .await?;
            info!("{user}, deleted user with id: {id}");
            Ok(())
        },
        (_, id) => {
            info!("{user} tried to delete account with id: {id}");
            Err(Error::NoPermission.into())
        }
    }
}

#[server]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    let user = user()?;
    match user.user_type {
        UserType::Inactive => {
            debug!("Inactive user {user} tried to access all users");
            Err(Error::NoPermission.into())
        },
        _ => {
            let db = db().await;
            let result = sqlx::query_as!(SqlUser, "SELECT * FROM users")
                .fetch_all(&db)
                .await?;
            Ok(result
               .into_iter()
               .map(|sqluser| sqluser.into())
               .collect()
            )
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserAndCount {
    pub id: i64,
    pub name: String,
    pub count: i64,
}

#[server]
pub async fn user_leaderboard() -> Result<Vec<UserAndCount>, ServerFnError> {
    let user = user()?;
    match user.user_type {
        UserType::Inactive => {
            debug!("Inactive user {user} tried to access leaderboard");
            Err(Error::NoPermission.into())
        },
        _ => {
            let db = db().await;
            let result = sqlx::query_as!(UserAndCount, r#"
                SELECT users.id, username AS name, COUNT(user_entries.entry_id) as count FROM users
                JOIN user_entries ON users.id == user_entries.user_id
                GROUP BY user_entries.user_id
                ORDER by count DESC
            "#)
                .fetch_all(&db)
                .await?;
            Ok(result)
        }
    }
}


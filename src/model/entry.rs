use serde::{Serialize, Deserialize};
use leptos::*;
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::{db, user::{UserType, get_user_by_username, get_user_by_id_tx, create_inactive_user}};
    use crate::auth::user;
    use sqlx::FromRow;
    use crate::errors::Error;
    use log::info;
}}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Entry {
    pub id: i64,
    pub how: String,
    pub created: chrono::NaiveDateTime,
}

pub fn validate_who(who: &str) -> bool {
    who.chars().all(|c| c.is_alphabetic() | c.is_whitespace() | (c == ','))
}

#[server(AddEntry)]
pub async fn add_entry(how: String, who: String) -> Result<i64, ServerFnError> {
    let user = user()?;
    match user.user_type {
        UserType::Reader | UserType::Inactive=> {
            info!("{user} does not have permission to add a new entry");
            Err(Error::NoPermission.into())
        }
        UserType::Admin | UserType::Writer => {
            let db = db().await;

            if !validate_who(&who) {
                info!("who field is invalid: {who}");
                return Err(Error::InvalidInput.into())
            }

            let mut transaction = db.begin().await?;
            let id = sqlx::query!("INSERT INTO entries (how) VALUES (?)", how)
                .execute(transaction.as_mut())
                .await?
                .last_insert_rowid();
            info!("{user} added entry: {how}");

            for maybe_username in who.split(",") {
                let maybe_username = maybe_username.trim();
                let entry_user = match get_user_by_username(maybe_username).await {
                    Ok(user) => user,
                    Err(_) => {
                        let id = create_inactive_user(&mut transaction, maybe_username).await?;
                        get_user_by_id_tx(&mut transaction, id).await?
                    },
                };

                sqlx::query!("INSERT INTO user_entries (user_id, entry_id) VALUES (?, ?)", entry_user.id, id)
                    .execute(transaction.as_mut())
                    .await?;
                info!("added {} as author for entry", entry_user.username);
            }
            transaction.commit().await?;
            Ok(id)
        },
    }
}

#[server]
pub async fn get_entry(id: i64) -> Result<Entry, ServerFnError> {
    user()?;
    let db = db().await;
    let result = sqlx::query_as!(Entry, "SELECT * FROM entries WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    Ok(result)
}

#[server]
pub async fn get_entries(query: String) -> Result<Vec<Entry>, ServerFnError> {
    user()?;
    let db = db().await;

    // TODO: check this, pretty sure this is secure, as the sql query is still prepared
    let query = format!("%{query}%");
    let result = sqlx::query_as!(Entry, "SELECT * FROM entries WHERE how LIKE ?", query)
        .fetch_all(&db)
        .await?;
    Ok(result)
}

#[server(DeleteEntry)]
pub async fn delete_entry(id: i64) -> Result<(), ServerFnError> {
    let user = user()?;
    match user.user_type {
        UserType::Reader | UserType::Inactive => {
            info!("{user} does not have permission to delete entry {id}");
            Err(Error::NoPermission.into())
        }
        UserType::Admin | UserType::Writer => {
            let db = db().await;
            sqlx::query!("DELETE FROM entries WHERE id = ?", id)
                .execute(&db)
                .await?;
            info!("{user} deleted entry with id: {id}");
            Ok(())
        },
    }
}


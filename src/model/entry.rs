use serde::{Serialize, Deserialize};
use leptos::*;
use cfg_if::cfg_if;

const WHO_DELIMITER: char = '+';

cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{
        model::db,
        auth::user,
        errors::Error,
        model::user::{
            UserType,
            get_user_by_username,
            get_user_by_id_tx,
            create_inactive_user,
            prepare_username,
        },
    };
    use sqlx::FromRow;
    use log::info;

    pub async fn create_entry(how: &str, who: &str) -> Result<i64, ServerFnError> {
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

        for maybe_username in who.split(WHO_DELIMITER) {
            let maybe_username = prepare_username(maybe_username); 
            let entry_user = match get_user_by_username(&maybe_username).await {
                Ok(user) => user,
                Err(_) => {
                    info!("{maybe_username} is not a user");
                    let id = create_inactive_user(&mut transaction, &maybe_username).await?;
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
    }
}}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Entry {
    pub id: i64,
    pub how: String,
    pub who: String,
    pub created: chrono::NaiveDateTime,
}

pub fn validate_who(who: &str) -> bool {
    who.chars().all(|c| c.is_alphabetic() | c.is_whitespace() | (c == WHO_DELIMITER))
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
            let result = create_entry(&how, &who).await?;
            info!("{user} added entry: {how}");
            Ok(result)
        }
    }
}

#[server]
pub async fn get_entries(query: String) -> Result<Vec<Entry>, ServerFnError> {
    user()?;
    let db = db().await;
    let query = format!("%{query}%");
    let delim = format!(" {WHO_DELIMITER} ");
    let result = sqlx::query_as!(Entry, r#"
            SELECT entries.id, created, how, GROUP_CONCAT(username, ?) AS who
            FROM entries
            JOIN user_entries ON entries.id == user_entries.entry_id
            JOIN users ON users.id == user_entries.user_id
            WHERE how LIKE ?
            GROUP BY how
            ORDER BY created DESC, entries.id DESC
            "#, delim, query)
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


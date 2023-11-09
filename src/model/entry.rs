use serde::{Serialize, Deserialize};
use leptos::*;
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::db;
    use sqlx::FromRow;
}}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Entry {
    pub id: i64,
    pub how: String,
    // pub who: String,
    pub created: chrono::NaiveDateTime,
}

#[server(AddEntry)]
pub async fn add_entry(how: String) -> Result<i64, ServerFnError> {
    // TODO: also add users to who field
    let db = db().await;
    let id = sqlx::query!("INSERT INTO entries (how) VALUES (?)", how)
        .execute(&db)
        .await?
        .last_insert_rowid();
    Ok(id)
}

#[server]
pub async fn get_entry(id: i64) -> Result<Entry, ServerFnError> {
    let db = db().await;
    let result = sqlx::query_as!(Entry, "SELECT * FROM entries WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    Ok(result)
}

#[server]
pub async fn get_entries() -> Result<Vec<Entry>, ServerFnError> {
    let db = db().await;
    let result = sqlx::query_as!(Entry, "SELECT * FROM entries")
        .fetch_all(&db)
        .await?;
    Ok(result)
}

#[server(DeleteEntry)]
pub async fn delete_entry(id: i64) -> Result<(), ServerFnError> {
    println!("{id}");
    let db = db().await;
    sqlx::query!("DELETE FROM entries WHERE id = ?", id)
        .execute(&db)
        .await?;
    Ok(())
}

// #[server]
// pub async fn update_entry(id: i64, how: String) -> Result<Entry, ServerFnError> {
//     let db = db().await;
//     let result = sqlx::query!("UPDATE entries SET how = ? WHERE id = ?", how, id)
//         .execute(&db)
//         .await?;
//     Ok(result)
// }


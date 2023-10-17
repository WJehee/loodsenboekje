use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use leptos::*;

use super::db;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Entry {
    pub id: i64,
    pub how: String,
    pub created: chrono::NaiveDateTime,
}

#[server]
pub async fn create_entry(id: i64, how: String) -> Result<i64> {
    let db = db();
    let id = sqlx::query!("INSERT INTO entry (how) VALUES (?)", entry.how)
        .execute(db)
        .await
        .map_err(|_| Error::DataBaseError)?
        .last_insert_rowid();
    Ok(id)
}

#[server]
pub async fn get_entry(id: i64) -> Result<Entry> {
    let db = db();
    let result = sqlx::query_as!(Entry, "SELECT * FROM entry WHERE id = ?", id)
        .fetch_one(db)
        .await
        .map_err(|_| Error::NotFound)?;
    Ok(result)
}

#[server]
pub async fn get_entries() -> Result<Vec<Entry>> {
    let db = db();
    let result = sqlx::query_as!(Entry, "SELECT * FROM entry")
        .fetch_all(db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(result)
}

#[server]
pub async fn delete_entry(id: i64) -> Result<()> {
    let db = db();
    sqlx::query!("DELETE FROM entry WHERE id = ?", id)
        .execute(db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(())
}

#[server]
pub async fn update_entry(id: i64, how: String) -> Result<Entry> {
    let db = db();
    sqlx::query!("UPDATE entry SET how = ?, created = ? WHERE id = ?", entry.how, entry.created, id)
        .execute(db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(entry)
}


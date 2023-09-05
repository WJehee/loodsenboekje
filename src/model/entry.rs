use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::model::ModelManager;
use crate::model::{Result, Error};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Entry {
    pub id: i64,
    pub how: String,
    pub created: chrono::NaiveDateTime,
}

pub struct EntryCreate {
    pub how: String,
}

pub struct EntryController;

impl EntryController {
    pub async fn create_entry(mm: &ModelManager, entry: EntryCreate) -> Result<i64> {
        let db = mm.db();
        let id = sqlx::query!("INSERT INTO entry (how) VALUES (?)", entry.how)
            .execute(db)
            .await
            .map_err(|_| Error::DataBaseError)?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_entry(mm: &ModelManager, id: i64) -> Result<Entry> {
        let db = mm.db();
        let result = sqlx::query_as!(Entry, "SELECT * FROM entry WHERE id = ?", id)
            .fetch_one(db)
            .await
            .map_err(|_| Error::NotFound)?;
        Ok(result)
    }

    pub async fn get_entries(mm: &ModelManager) -> Result<Vec<Entry>> {
        let db = mm.db();
        let result = sqlx::query_as!(Entry, "SELECT * FROM entry")
            .fetch_all(db)
            .await
            .map_err(|_| Error::DataBaseError)?;
        Ok(result)
    }

    pub async fn delete_entry(mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();
        sqlx::query!("DELETE FROM entry WHERE id = ?", id)
            .execute(db)
            .await
            .map_err(|_| Error::DataBaseError)?;
        Ok(())
    }

    pub async fn update_entry(mm: &ModelManager, entry: Entry, id: i64) -> Result<Entry> {
        let db = mm.db();
        sqlx::query!("UPDATE entry SET how = ?, created = ? WHERE id = ?", entry.how, entry.created, id)
            .execute(db)
            .await
            .map_err(|_| Error::DataBaseError)?;
        Ok(entry)
    }
}


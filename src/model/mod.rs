use sqlx::SqlitePool;

mod error;
mod user;
mod entry;

pub use self::error::{Result, Error};

#[derive(Clone)]
pub struct ModelManager {
    db: SqlitePool,
}

impl ModelManager {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub (in crate::model) fn db(&self) -> &SqlitePool {
        &self.db
    }
}

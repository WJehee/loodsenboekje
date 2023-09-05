use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::model::{Result, Error, ModelManager};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}

pub struct UserCreate {
    name: String,
}

pub struct UserController;

impl UserController {
    pub async fn create_user(mm: &ModelManager, user: UserCreate) -> Result<i64> {
        let db = mm.db();
        let id: i64 = sqlx::query!("INSERT INTO users (name) VALUES (?)", user.name) 
            .execute(db)
            .await
            .map_err(|_| Error::DataBaseError)?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_user(mm: &ModelManager, id: i64) -> Result<User> {
        let db = mm.db();
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(db)
            .await
            .map_err(|_| Error::NotFound)?;
        Ok(result)
    }

    pub async fn get_users(mm: &ModelManager) -> Result<Vec<User>> {
        let db = mm.db();
        let result = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(db)
            .await
            .map_err(|_| Error::DataBaseError)?;
        Ok(result)
    }

    pub async fn delete_user(mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(db)
            .await
            .map_err(|_| Error::DataBaseError)?;
        Ok(())
    }

    pub async fn update_user(mm: &ModelManager, id: i64, user: UserCreate) -> Result<User> {
        let db = mm.db();
        sqlx::query!("UPDATE users SET name = ?, WHERE id = ?", user.name, id)
            .execute(&db)
            .await
            .map_err(|_| Error::DataBaseError)?;
        Ok(User {
            id,
            name: user.name,
        })
    }
}

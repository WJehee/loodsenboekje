use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Entry {
    pub id: u64,
    pub how: String,
    pub when: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub password: String,
}


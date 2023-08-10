use crate::{error::{Result, Error}, model::User};

use axum::{Router, extract::State, Json, routing::get};
use serde_json::Value;
use sqlx::SqlitePool;

pub fn routes(db: SqlitePool) -> Router {
    Router::new()
        .route("/users", get(list).post(create))
        .with_state(db)
}

async fn create(State(db): State<SqlitePool>) -> Result<Json<Value>> {
    let id: i64 = sqlx::query!("INSERT INTO users (name, password) VALUES (?, ?)", "test", "test")
        .execute(&db)
        .await
        .map_err(|_| Error::DataBaseError)?
        .last_insert_rowid();
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn list(State(db): State<SqlitePool>) -> Result<Json<Value>> {
    let result = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}


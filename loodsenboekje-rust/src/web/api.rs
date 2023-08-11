use crate::{error::{Result, Error}, model::User};

use axum::{Router, extract::{State, Path}, Json, routing::get};
use serde_json::Value;
use sqlx::SqlitePool;

pub fn routes(db: SqlitePool) -> Router {
    Router::new()
        .route("/users", get(list_users)
            .post(create_user))
        .route("/users/:id", get(get_user)
            .post(update_user)
            .delete(delete_user))
        .with_state(db)
}

async fn list_users(State(db): State<SqlitePool>) -> Result<Json<Value>> {
    let result = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}

async fn create_user(State(db): State<SqlitePool>, Json(user): Json<User>) -> Result<Json<Value>> {
    // TODO: if we end up using passwords instead of a simple .env variable, hash it before inserting
    let id: i64 = sqlx::query!("INSERT INTO users (name, password) VALUES (?, ?)", user.name, user.password) 
        .execute(&db)
        .await
        .map_err(|_| Error::DataBaseError)?
        .last_insert_rowid();
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn get_user(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Value>> {
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db)
        .await
        .map_err(|_| Error::NotFound)?;
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}

async fn delete_user(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Value>> {
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn update_user(State(db): State<SqlitePool>, Path(id): Path<i64>, Json(user): Json<User>) -> Result<Json<Value>> {
    sqlx::query!("UPDATE users SET name = ?, password = ? WHERE id = ?", user.name, user.password, id)
        .execute(&db)
        .await
        .map_err(|_| Error::DataBaseError)?;
    Ok(Json(serde_json::to_value(id).unwrap()))
}


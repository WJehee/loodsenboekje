use crate::{error::{Result, Error}, model:: ModelManager};

use axum::{Router, extract::{State, Path}, Json, routing::get};
use serde_json::Value;

pub fn routes(model: ModelManager) -> Router {
    Router::new()
        .route("/users", get(list_users)
            .post(create_user))
        .route("/users/:id", get(get_user)
            .post(update_user)
            .delete(delete_user))
        .route("/entries", get(list_entries)
            .post(create_entry))
        .route("/entries/:id", get(get_entry)
            .post(update_entry)
            .delete(delete_entry))
        .with_state(model)
}

async fn list_users(State(db): State<ModelManager>) -> Result<Json<Value>> {
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}

async fn create_user(State(db): State<ModelManager>, Json(user): Json<User>) -> Result<Json<Value>> {
    // TODO: if we end up using passwords instead of a simple .env variable, hash it before inserting
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn get_user(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}

async fn delete_user(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn update_user(State(db): State<ModelManager>, Path(id): Path<i64>, Json(user): Json<User>) -> Result<Json<Value>> {
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn list_entries(State(db): State<ModelManager>) -> Result<Json<Value>> {
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}

async fn create_entry(State(db): State<ModelManager>, Json(entry): Json<Entry>) -> Result<Json<Value>> {
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn get_entry(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    Ok(Json(
        serde_json::to_value(result).unwrap()
    ))
}

async fn delete_entry(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn update_entry(State(model): State<ModelManager>, Path(id): Path<i64>, Json(entry): Json<Entry>) -> Result<Json<Value>> {
    await model.update_entry(entry, id)
}


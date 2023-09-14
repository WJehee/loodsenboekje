use crate::{error::{Result}, model:: ModelManager};
use crate::model::user::{UserCreate, UserController};
use crate::model::entry::{Entry, EntryCreate, EntryController};

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
    let users = UserController::list(&db).await?;
    Ok(Json(serde_json::to_value(users).unwrap()))
}

async fn create_user(State(db): State<ModelManager>, Json(user): Json<UserCreate>) -> Result<Json<Value>> {
    // TODO: if we end up using passwords instead of a simple .env variable, hash it before inserting
    let id = UserController::create(&db, user).await?;
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn get_user(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    let user = UserController::get(&db, id).await?;
    Ok(Json(serde_json::to_value(user).unwrap()))
}

async fn delete_user(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    UserController::delete(&db, id).await?;
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn update_user(State(db): State<ModelManager>, Path(id): Path<i64>, Json(user): Json<UserCreate>) -> Result<Json<Value>> {
    let user = UserController::update(&db, id, user).await?;
    Ok(Json(serde_json::to_value(user).unwrap()))
}

async fn list_entries(State(db): State<ModelManager>) -> Result<Json<Value>> {
    let entries = EntryController::list(&db).await?;
    Ok(Json(serde_json::to_value(entries).unwrap()))
}

async fn create_entry(State(db): State<ModelManager>, Json(entry): Json<EntryCreate>) -> Result<Json<Value>> {
    let id = EntryController::create(&db, entry).await?;
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn get_entry(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    let entry = EntryController::get(&db, id).await?;
    Ok(Json(serde_json::to_value(entry).unwrap()))
}

async fn delete_entry(State(db): State<ModelManager>, Path(id): Path<i64>) -> Result<Json<Value>> {
    EntryController::delete(&db, id).await?;
    Ok(Json(serde_json::to_value(id).unwrap()))
}

async fn update_entry(State(db): State<ModelManager>, Path(id): Path<i64>, Json(entry): Json<Entry>) -> Result<Json<Value>> {
    let entry = EntryController::update(&db, id, entry).await?;
    Ok(Json(serde_json::to_value(entry).unwrap()))
}


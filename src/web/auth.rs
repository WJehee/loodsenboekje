use crate::error::{Result, Error};

use axum::{Router, routing::{get, post}, Json};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use super::AUTH_COOKIE_NAME;

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> { 
    // TODO: do login better
    if payload.username != "admin" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }
    cookies.add(Cookie::new(AUTH_COOKIE_NAME, "admin"));

    Ok(Json(json!({
        "success": true
    })))
}

async fn logout(cookies: Cookies) -> Result<Json<Value>> {
    cookies.remove(Cookie::new(AUTH_COOKIE_NAME, ""));
    Ok(Json(json!({
        "success": true
    })))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}


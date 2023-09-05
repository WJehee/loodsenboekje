use std::net::SocketAddr;

use axum::{Router, routing::get_service};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use sqlx::SqlitePool;

mod model;
mod web;
mod error;

#[tokio::main]
async fn main() {
    let db = SqlitePool::connect("sqlite.db").await.unwrap();
    let model = model::ModelManager::new(db.clone());
    let router = Router::new()
        .merge(web::auth::routes())
        .nest("/api", web::api::routes(model))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static())
    ;
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on address: {}", addr);
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("static")))
}


use std::net::SocketAddr;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let router = Router::new().route(
        "/", get(|| async {
            "Hello, world!"
        })
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on address: {}", addr);
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}

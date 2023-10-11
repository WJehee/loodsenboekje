
// mod model;
// mod web;
// mod error;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    use tower_cookies::CookieManagerLayer;
    use sqlx::SqlitePool;
    use loodsenboekje::app::*;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();
    let routes = generate_route_list(App);

    let db = SqlitePool::connect("sqlite.db").await.unwrap();
    let router = Router::new()
        // .merge(web::auth::routes())
        // .nest("/api", web::api::routes(model.clone()))
        .layer(CookieManagerLayer::new())
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options)
        .fallback_service(routes_static(&site_root))
    ;
    println!("listening on address: {}", addr);
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}

#[cfg(feature = "ssr")]
fn routes_static(root: &str) -> axum::Router {
    use tower_http::services::ServeDir;
    use axum::routing::get_service;
    axum::Router::new().nest_service("/", get_service(ServeDir::new(root)))
}


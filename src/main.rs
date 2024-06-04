pub mod app;
pub mod components;
pub mod model;
pub mod auth;
pub mod errors;

use std::{env, fs::File};

use axum::{
    body::Body as AxumBody, extract::{FromRef, State}, http::Request, response::{IntoResponse, Response}, routing::get, Router
};
use tower_sessions::{MemoryStore, SessionManagerLayer, Session};

use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};

use dotenvy::dotenv;
use leptos_router::RouteListing;
use simplelog::*;

use loodsenboekje::app::*;
use sqlx::SqlitePool;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
    pub pool: SqlitePool,
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    session: Session,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(move || {
        provide_context(app_state.pool.clone());
        provide_context(session.clone());
    }, request).await
}

#[axum::debug_handler]
async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    session: Session,
    req: Request<AxumBody>
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.pool.clone());
            provide_context(session.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

fn routes_static(root: &str) -> axum::Router {
    use tower_http::services::ServeDir;
    use axum::routing::get_service;
    axum::Router::new().nest_service("/", get_service(ServeDir::new(root)))
}

#[tokio::main]
async fn main() {
    let _ = dotenv();
    env::var("READ_PASSWORD").expect("Expected READ_PASSWORD to be set");
    env::var("WRITE_PASSWORD").expect("Expected WRITE_PASSWORD to be set");
    env::var("ADMIN_PASSWORD").expect("Expected ADMIN_PASSWORD to be set");
    let data_dir = env::var("DATA_DIR").expect("Expected DATA_DIR to be set");

    let db_pool = SqlitePool::connect(&format!("{data_dir}/sqlite.db")).await.expect("Failed to connect to database");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        ;

    let conf = get_configuration(None).await.expect("Failed to get leptos configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        routes: routes.clone(),
        pool: db_pool,
    };

    let router = Router::new()
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback_service(routes_static(&site_root))
        .with_state(app_state)
        .layer(session_layer)
        ;

    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            ConfigBuilder::new()
            .add_filter_allow("loodsenboekje".to_string())
            .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
            .add_filter_allow("loodsenboekje".to_string())
            .build(),
            File::create(&format!("{data_dir}/loodsenboekje.log")).expect("Failed to open log file"),
        ),
    ]);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .expect("Failed to start server");
    }


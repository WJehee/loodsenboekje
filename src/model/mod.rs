use cfg_if::cfg_if;

pub mod user;
pub mod entry;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_session::{Session, SessionNullPool};
    use leptos::{ServerFnError, use_context};

    pub async fn db() -> SqlitePool {
        SqlitePool::connect("sqlite.db").await.unwrap()
    }

    pub fn session() -> Result<Session<SessionNullPool>, ServerFnError> {
        use_context::<Session<SessionNullPool>>()
            .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))
    }
}}


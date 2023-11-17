use cfg_if::cfg_if;

pub mod user;
pub mod entry;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;

    pub async fn db() -> SqlitePool {
        SqlitePool::connect("sqlite.db").await.unwrap()
    }
}}


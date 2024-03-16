use cfg_if::cfg_if;

pub mod user;
pub mod entry;

cfg_if! { if #[cfg(feature = "ssr")] {
    use std::env;
    use sqlx::SqlitePool;

    pub async fn db() -> SqlitePool {
        let data_dir = env::var("DATA_DIR").expect("DATA_DIR to be set");
        SqlitePool::connect(&format!("{data_dir}/sqlite.db")).await.expect("Failed to connect to database")
    }
}}


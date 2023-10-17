use sqlx::SqlitePool;

pub mod user;
pub mod entry;

async fn db() -> SqlitePool {
    SqlitePool::connect("sqlite.db").await.unwrap()
}


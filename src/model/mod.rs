pub mod user;
pub mod entry;

#[cfg(feature="ssr")]
use leptos::ServerFnError;
#[cfg(feature="ssr")]
use sqlx::SqlitePool;

#[cfg(feature="ssr")]
pub async fn db() -> Result<SqlitePool, ServerFnError> {
    use leptos::use_context;
    use log::warn;
    
    use crate::errors::Error;

    use_context::<SqlitePool>()
        .ok_or_else(|| {
            warn!("Failed to get database pool");
            Error::Database.into()
        })
}

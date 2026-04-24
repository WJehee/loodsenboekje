//! Authelia trusted-header authentication.
//!
//! When `AUTH_MODE=authelia`, the upstream reverse proxy authenticates the
//! request with Authelia and forwards the identity as HTTP headers. We read
//! `Remote-User` and `Remote-Groups`, look up or auto-provision a row in the
//! `users` table, and derive the `UserType` from the Authelia group
//! membership.
//!
//! SECURITY: this is only safe when the app is not directly reachable and the
//! reverse proxy strips client-supplied `Remote-*` headers. See
//! docs/authelia-integration.md.

#[cfg(feature="ssr")]
use axum::http::HeaderMap;
#[cfg(feature="ssr")]
use leptos::ServerFnError;
#[cfg(feature="ssr")]
use log::{debug, info, warn};

#[cfg(feature="ssr")]
use crate::errors::Error;
#[cfg(feature="ssr")]
use crate::model::db;
#[cfg(feature="ssr")]
use crate::model::user::{
    get_user_by_username, prepare_username, User, UserType,
};

#[cfg(feature="ssr")]
const HEADER_USER: &str = "remote-user";
#[cfg(feature="ssr")]
const HEADER_GROUPS: &str = "remote-groups";
#[cfg(feature="ssr")]
const HEADER_NAME: &str = "remote-name";
#[cfg(feature="ssr")]
const HEADER_EMAIL: &str = "remote-email";

#[cfg(feature="ssr")]
const DEFAULT_GROUP_ADMIN: &str = "loodsenboekje-admin";
#[cfg(feature="ssr")]
const DEFAULT_GROUP_WRITER: &str = "loodsenboekje-writer";
#[cfg(feature="ssr")]
const DEFAULT_GROUP_READER: &str = "loodsenboekje-reader";

/// Parsed Authelia headers, injected into Leptos context per-request by the
/// Axum handlers in `main.rs`.
#[cfg(feature="ssr")]
#[derive(Clone, Debug, Default)]
pub struct AutheliaHeaders {
    pub user: Option<String>,
    pub groups: Vec<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[cfg(feature="ssr")]
impl AutheliaHeaders {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let get = |name: &str| {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        let groups = get(HEADER_GROUPS)
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect();
        Self {
            user: get(HEADER_USER).filter(|s| !s.is_empty()),
            groups,
            name: get(HEADER_NAME).filter(|s| !s.is_empty()),
            email: get(HEADER_EMAIL).filter(|s| !s.is_empty()),
        }
    }
}

/// Resolve a `User` from Authelia headers, auto-provisioning / refreshing the
/// DB row as needed.
#[cfg(feature="ssr")]
pub async fn resolve_user_from_headers(
    headers: &AutheliaHeaders,
) -> Result<User, ServerFnError> {
    let Some(raw_username) = headers.user.as_deref() else {
        debug!("Authelia mode: no Remote-User header on request");
        return Err(Error::NotLoggedIn.into());
    };

    let username = prepare_username(raw_username);
    let user_type = map_groups_to_user_type(&headers.groups);

    let db = db().await?;
    match get_user_by_username(&username).await {
        Ok(existing) => {
            let desired = user_type.clone() as i64;
            if existing.user_type != desired {
                // Keep the DB in sync with Authelia group membership so that
                // role changes propagate without requiring a manual step.
                sqlx::query!(
                    "UPDATE users SET user_type = ? WHERE id = ?",
                    desired,
                    existing.id,
                )
                .execute(&db)
                .await
                .map_err(|_| -> ServerFnError { Error::Database.into() })?;
                info!(
                    "Authelia: updated user_type for '{username}' (id {}) to {desired}",
                    existing.id
                );
            }
            let mut user: User = existing.into();
            user.user_type = user_type;
            Ok(user)
        }
        Err(_) => {
            let empty_password = String::new();
            let user_type_i = user_type.clone() as i64;
            let id: i64 = sqlx::query!(
                "INSERT INTO users (username, password, user_type) VALUES (?, ?, ?)",
                username,
                empty_password,
                user_type_i,
            )
            .execute(&db)
            .await
            .map_err(|e| -> ServerFnError {
                warn!("Authelia: failed to auto-provision '{username}': {e}");
                Error::Database.into()
            })?
            .last_insert_rowid();
            info!("Authelia: auto-provisioned user '{username}' with id {id}");
            Ok(User {
                id,
                name: username,
                user_type,
            })
        }
    }
}

#[cfg(feature="ssr")]
fn map_groups_to_user_type(groups: &[String]) -> UserType {
    let admin = env_or(DEFAULT_GROUP_ADMIN, "AUTHELIA_GROUP_ADMIN");
    let writer = env_or(DEFAULT_GROUP_WRITER, "AUTHELIA_GROUP_WRITER");
    let reader = env_or(DEFAULT_GROUP_READER, "AUTHELIA_GROUP_READER");

    // Highest privilege wins.
    if groups.iter().any(|g| g == &admin) {
        UserType::Admin
    } else if groups.iter().any(|g| g == &writer) {
        UserType::Writer
    } else if groups.iter().any(|g| g == &reader) {
        UserType::Reader
    } else {
        UserType::Inactive
    }
}

#[cfg(feature="ssr")]
fn env_or(default: &str, var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_owned())
}

pub mod authelia;

use leptos::{ServerFnError, server};
use serde::{Deserialize, Serialize};

#[cfg(feature="ssr")]
use tower_sessions::Session;
#[cfg(feature="ssr")]
use leptos::use_context;
#[cfg(feature="ssr")]
use log::{debug, info, warn};

#[cfg(feature="ssr")]
use crate::model::user::{
    get_user_by_username,
    prepare_username,
};
use crate::model::user::User;
#[cfg(feature="ssr")]
use crate::errors::Error;

#[cfg(feature="ssr")]
use self::authelia::{AutheliaHeaders, resolve_user_from_headers};

pub const USER_STRING: &str = "user";

/// Which authentication scheme the server is running under.
///
/// Selected at startup from the `AUTH_MODE` env var. `Local` preserves the
/// original username/password flow; `Authelia` trusts headers set by an
/// upstream reverse proxy running Authelia (see docs/authelia-integration.md).
///
/// This is `Serialize`/`Deserialize` so it can be shipped to the client via a
/// server function — the UI uses it to hide the login / register / logout
/// affordances when Authelia is in charge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AuthMode {
    Local,
    Authelia,
}

impl AuthMode {
    #[cfg(feature="ssr")]
    pub fn from_env() -> Self {
        match std::env::var("AUTH_MODE").ok().as_deref() {
            Some("authelia") | Some("Authelia") | Some("AUTHELIA") => Self::Authelia,
            _ => Self::Local,
        }
    }
}

#[cfg(feature="ssr")]
pub fn auth_mode() -> AuthMode {
    use_context::<AuthMode>().unwrap_or(AuthMode::Local)
}

#[server(GetAuthMode)]
pub async fn get_auth_mode() -> Result<AuthMode, ServerFnError> {
    Ok(auth_mode())
}

#[cfg(feature="ssr")]
pub async fn user() -> Result<User, ServerFnError> {
    match auth_mode() {
        AuthMode::Local => user_from_session().await,
        AuthMode::Authelia => user_from_headers().await,
    }
}

#[cfg(feature="ssr")]
async fn user_from_session() -> Result<User, ServerFnError> {
    let session = session()?;
    match session.get::<User>(USER_STRING).await {
        Ok(Some(user)) => Ok(user),
        _ => Err(Error::NotLoggedIn.into())
    }
}

#[cfg(feature="ssr")]
async fn user_from_headers() -> Result<User, ServerFnError> {
    let headers = use_context::<AutheliaHeaders>()
        .ok_or_else(|| {
            warn!("Authelia mode but no AutheliaHeaders in context");
            let err: ServerFnError = Error::NotLoggedIn.into();
            err
        })?;
    resolve_user_from_headers(&headers).await
}

#[cfg(feature="ssr")]
pub fn session() -> Result<Session, ServerFnError> {
    use_context::<Session>()
        .ok_or_else(|| {
            warn!("Failed to get session...");
            Error::NotLoggedIn.into()
        })
}

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use bcrypt::verify;

    if auth_mode() == AuthMode::Authelia {
        // In Authelia mode login is handled by the reverse proxy; the form
        // is hidden, but guard the endpoint anyway.
        leptos_axum::redirect("/");
        return Ok(());
    }

    debug!("Trying to log in");

    let username = prepare_username(&username);
    let sqluser = get_user_by_username(&username).await?;

    match verify(password, &sqluser.password)? {
        true => {
            let user: User = sqluser.into();
            info!("{user} logged in");
            let session = session()?;
            let _ = session.insert(USER_STRING, user).await;

            leptos_axum::redirect("/");
            Ok(())
        },
        false => {
            info!("{username} tried to log in but failed");
            Err(Error::InvalidInput.into())
        }
    }
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    if auth_mode() == AuthMode::Authelia {
        // Delegate to Authelia's own logout endpoint (configured on the
        // reverse proxy). We just bounce to the root.
        leptos_axum::redirect("/");
        return Ok(());
    }

    debug!("Trying to log out");
    let session = session()?;

    if let Ok(Some(user)) = session.get::<User>(USER_STRING).await {
        let _ = session.delete().await;
        info!("{user} logged out");
    };

    leptos_axum::redirect("/login");
    Ok(())
}

#[server(CurrentUser)]
pub async fn current_user() -> Result<Option<User>, ServerFnError> {
    match user().await {
        Ok(user) => {
            debug!("current user: {user}");
            Ok(Some(user))
        },
        Err(_) => {
            debug!("not logged in");
            Ok(None)
        }
    }
}

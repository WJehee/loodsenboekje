use leptos::{ServerFnError, server};

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

pub const USER_STRING: &str = "user";

#[cfg(feature="ssr")]
pub async fn user() -> Result<User, ServerFnError> {
    let session = session()?;
    match session.get::<User>(USER_STRING).await {
        Ok(Some(user)) => Ok(user),
        _ => Err(Error::NotLoggedIn.into())
    }
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


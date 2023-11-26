use leptos::{ServerFnError, server};
use cfg_if::cfg_if;

use crate::model::user::User;

pub const USER_STRING: &str = "user";

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::model::user::get_user_by_username;
        use axum_session::{Session, SessionNullPool};
        use leptos::use_context;
        use crate::errors::Error;
        use log::{info, warn};

        pub fn user() -> Result<User, ServerFnError> {
            let session = session()?;
            match session.get::<User>(USER_STRING) {
                Some(user) => Ok(user),
                None => {
                    warn!("Failed to extract user but session was available");
                    Err(Error::NotLoggedIn.into())
                }
            }
        }

        pub fn session() -> Result<Session<SessionNullPool>, ServerFnError> {
            use_context::<Session<SessionNullPool>>()
                .ok_or_else(|| {
                    info!("Failed to get session, user not logged in");
                    Error::NotLoggedIn.into()
                })
        }
    }
}

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    use bcrypt::verify;

    let sqluser = get_user_by_username(&username).await?;

    match verify(password, &sqluser.password)? {
        true => {
            let user: User = sqluser.into();
            info!("{user} logged in");
            let session= session()?;
            session.set_store(true);
            session.set(USER_STRING, user);

            leptos_axum::redirect("/");
            Ok(())
        },
        false => Err(Error::InvalidInput.into())
    }
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    let session = session()?;
    if let Some(user) = session.get::<User>(USER_STRING) {
        session.destroy();
        info!("{user} logged out");
    };

    leptos_axum::redirect("/login");
    Ok(())
}

#[server(CurrentUser)]
pub async fn current_user() -> Result<Option<User>, ServerFnError> {
    match user() {
        Ok(user) => {
            info!("current user: {user}");
            Ok(Some(user))
        },
        Err(_) => {
            info!("not logged in");
            Ok(None)
        }
    }
}


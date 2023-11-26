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

        pub fn user() -> Result<User, ServerFnError> {
            let session = session()?;
            match session.get::<User>(USER_STRING) {
                Some(user) => Ok(user),
                None => {
                    eprintln!("Failed to extract user");
                    Err(Error::NotLoggedIn.into())
                }
            }
        }

        pub fn session() -> Result<Session<SessionNullPool>, ServerFnError> {
            use_context::<Session<SessionNullPool>>()
                .ok_or_else(|| {
                    eprintln!("Failed to get session");
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
            println!("{user} logged in");
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
        println!("{user} logged out");
    };

    leptos_axum::redirect("/login");
    Ok(())
}

#[server(CurrentUser)]
pub async fn current_user() -> Result<Option<User>, ServerFnError> {
    match user() {
        Ok(user) => {
            println!("current user: {user}");
            Ok(Some(user))
        },
        Err(_) => {
            println!("not logged in");
            Ok(None)
        }
    }
}


use leptos::{ServerFnError, server};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::model::session;
    }
}

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    use super::model::{db, user::SqlUser};
    use bcrypt::verify;
    
    let db = db().await;
    let sqluser = sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(&db)
        .await?;

    let session= session()?;
    match verify(password, &sqluser.password)? {
        true => {
            println!("User logged in as {username}");

            session.set_store(true);
            session.set("username", username);

            leptos_axum::redirect("/");
            Ok(())
        },
        // TODO: error handling
        false => Err(ServerFnError::ServerError("Password does not match".to_string()))
    }
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    println!("logout attempt");

    let session = session().map_err(|err| {
        eprintln!("Unable to retrieve session, err: {err}");
        ServerFnError::ServerError("session error".into())
    })?;
    if let Some(username) = session.get::<String>("username") {
        println!("{username} logged out");
    };

    leptos_axum::redirect("/login");
    Ok(())
}

#[server]
pub async fn current_user() -> Result<Option<String>, ServerFnError> {
    let session = session()?;

    match session.get::<String>("username") {
        Some(user) => {
            println!("current username: {user:?}");
            Ok(Some(user))
        },
        None => {
            leptos_axum::redirect("/login");
            Ok(None)
        }
    }
}


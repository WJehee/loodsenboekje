use leptos::{ServerFnError, server};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::model::session;
        use super::model::user::get_user_by_username;
    }
}

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    use bcrypt::verify;
    
    let sqluser = get_user_by_username(&username).await?;

    match verify(password, &sqluser.password)? {
        true => {
            println!("User logged in as {username}");

            let session= session()?;
            session.set_store(true);
            session.set("user_id", sqluser.id);
            session.set("username", username);

            leptos_axum::redirect("/");
            Ok(())
        },
        false => Err(ServerFnError::ServerError("Password does not match".to_string()))
    }
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    println!("logout attempt");

    let session = session()?;
    if let Some(username) = session.get::<String>("username") {
        // TODO: delete session
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


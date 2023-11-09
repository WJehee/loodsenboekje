use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] { 
    use super::model::{db, user::SqlUser};
}}

use leptos::{ServerFnError, server};

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    println!("login attempt: {username}:{password}");
    use bcrypt::verify;
    
    let db = db().await;
    let sqluser = sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(&db)
        .await?;
    match verify(password, &sqluser.password)? {
        true => Ok(()),
        false => Err(ServerFnError::ServerError("Password does not match".to_string()))
    }
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    // TODO: remove user session
    Ok(())
}


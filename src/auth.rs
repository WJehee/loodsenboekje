use leptos::{ServerFnError, server};

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    use super::model::{db, user::SqlUser};
    use bcrypt::verify;
    
    let db = db().await;
    let sqluser = sqlx::query_as!(SqlUser, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(&db)
        .await?;
    match verify(password, &sqluser.password)? {
        // TODO: create session for the user
        // TODO: redirect to home page
        true => Ok(()),
        // TODO: error handling
        false => Err(ServerFnError::ServerError("Password does not match".to_string()))
    }
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    // TODO: remove user session
    // TODO: redirect to login page
    Ok(())
}


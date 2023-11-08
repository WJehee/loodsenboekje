use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] { 

}}

use leptos::{ServerFnError, server};

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    println!("{}, {}", username, password);
    // TODO: do login better
    if username != "admin" || password != "admin" {
        return Err(ServerFnError::ServerError("Invalid credentials".into()))
    }
    Ok(())
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    Ok(())
}


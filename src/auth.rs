// use cfg_if::cfg_if;
// cfg_if! { if #[cfg(feature = "ssr")] { 
// }}

use leptos::{ServerFnError, server};

#[server(Login)]
async fn login(username: String, password: String) -> Result<(), ServerFnError> { 
    // TODO: check username and password
    Ok(())
}

#[server(Logout)]
async fn logout() -> Result<(), ServerFnError> {
    // TODO: remove user session
    Ok(())
}


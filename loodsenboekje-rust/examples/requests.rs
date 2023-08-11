use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/").await?.print().await?;

    // Login/logout
    hc.do_post("/login", json!({
       "username": "admin",
       "password": "admin"
    })).await?.print().await?;
    hc.do_get("/logout").await?.print().await?;

    // CRUD users
    hc.do_post("/api/users", json!({
        "name": "test",
        "password": "test",
    })).await?.print().await?;
    hc.do_get("/api/users").await?.print().await?;
    hc.do_get("/api/users/1").await?.print().await?;
    hc.do_delete("/api/users/1").await?.print().await?;
    hc.do_get("/api/users/1").await?.print().await?;
    hc.do_put("/api/users/2", json!({
        "name": "test1",
        "password": "test1"
    })).await?.print().await?;

    Ok(())
}

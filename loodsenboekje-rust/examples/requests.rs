use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/").await?.print().await?;
    hc.do_post("/login", json!({
       "username": "admin",
       "password": "admin"
    })).await?.print().await?;
    hc.do_get("/logout").await?.print().await?;
    hc.do_get("/").await?.print().await?;

    Ok(())
}

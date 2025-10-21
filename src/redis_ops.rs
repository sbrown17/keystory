use anyhow::Result;
use redis::AsyncCommands;
use tokio;

use crate::git_ops;

pub async fn get_key(key: String) -> Result<(String, String)> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_async_connection().await?;
    let value: Option<String> = conn.get(&key).await.ok();
    Ok((key, value.unwrap_or_default()))
}

pub async fn set_and_commit(key: String, value: String) -> Result<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_async_connection().await?;
    conn.set::<_, _, ()>(key.clone(), value.clone()).await?;

    // Save the key locally and commit it
    std::fs::write("keystory_data.txt", format!("{key}={value}"))?;

    tokio::task::spawn_blocking(move || {
        git_ops::commit_change("keystory_data.txt", &format!("Update key {key}"))
    })
    .await??;

    Ok(())
}
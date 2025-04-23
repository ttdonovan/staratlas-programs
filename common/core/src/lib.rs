use anyhow::Result;
use solana_client::nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;

use sqlx::{
    Pool,
    sqlite::{Sqlite, SqlitePoolOptions},
};

pub async fn solana_pubsub_client(ws_url: &str) -> Result<PubsubClient> {
    let client = PubsubClient::new(ws_url).await?;
    Ok(client)
}

pub async fn solana_rpc_client(rpc_url: &str) -> Result<RpcClient> {
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    Ok(client)
}

pub async fn sqlite_pool(database_url: &str) -> Result<Pool<Sqlite>> {
    let pool = SqlitePoolOptions::new().connect(database_url).await?;

    sqlx::query(
        r#"
        PRAGMA synchronous=NORMAL;
        PRAGMA journal_mode=WAL;
        PRAGMA temp_store=MEMORY;
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

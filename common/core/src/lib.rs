use anchor_lang::Discriminator;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};

// Re-export sqlx
pub use sqlx;

use sqlx::{
    Pool,
    sqlite::{Sqlite, SqlitePoolOptions},
};

pub async fn solana_pubsub_client(ws_url: &str) -> anyhow::Result<PubsubClient> {
    let client = PubsubClient::new(ws_url).await?;
    Ok(client)
}

pub async fn solana_rpc_client(rpc_url: &str) -> anyhow::Result<RpcClient> {
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    Ok(client)
}

pub async fn get_program_accounts<T: Discriminator>(
    client: &RpcClient,
    program_id: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let discrim = T::DISCRIMINATOR;
    let config = RpcProgramAccountsConfig {
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
            0,
            MemcmpEncodedBytes::Bytes(discrim.into()),
        ))]),
        ..Default::default()
    };

    let accounts = client
        .get_program_accounts_with_config(program_id, config)
        .await?;

    Ok(accounts)
}

pub async fn sqlite_pool(database_url: &str) -> anyhow::Result<Pool<Sqlite>> {
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

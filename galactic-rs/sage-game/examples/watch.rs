use dotenv::dotenv;
use futures_util::StreamExt;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use tokio::signal;

use common_core::{solana_pubsub_client, sqlite_pool, sqlx};
use staratlas_sage::ID as SAGE_ID;

const INSERT_OR_REPLACE_SAGE_ACCOUNTS_SQL: &str = r#"
INSERT OR REPLACE INTO staratlas_sage_accounts
    (pubkey, owner, data, space, slot)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5
);
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = sqlite_pool(&database_url).await?;
    sqlx::migrate!("./../../migrations/sage").run(&pool).await?;

    let ws_url = dotenv::var("WS")?;
    let pubsub_client = solana_pubsub_client(&ws_url).await?;

    let program_accounts_config = RpcProgramAccountsConfig {
        // filters: Option<Vec<RpcFilterType>>,
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        ..Default::default()
    };

    let (mut accounts, unsubscriber) = pubsub_client
        .program_subscribe(&SAGE_ID, Some(program_accounts_config))
        .await?;

    println!("Watching for Star Atlas SAGE program account updates... (Press Ctrl+C to exit)");

    let mut counter = 0;

    loop {
        tokio::select! {
            // Handel Ctrl+C signal
            _ = signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down gracefully...");
                break;
            }
            // Process account updates
            account_option = accounts.next() => {
                match account_option {
                    Some(response) => {
                        let _ = sqlx::query(INSERT_OR_REPLACE_SAGE_ACCOUNTS_SQL)
                            .bind(&response.value.pubkey)
                            .bind(response.value.account.owner)
                            .bind(response.value.account.data.decode().unwrap_or(vec![]))
                            .bind(response.value.account.space.unwrap_or(0) as i64)
                            .bind(response.context.slot as i64)
                            .execute(&pool)
                            .await?;

                        counter += 1;
                        println!("Processed account #{}: {}", counter, response.value.pubkey);
                    }
                    None => {
                        println!("Account stream ended");
                        break;
                    }
                }
            }
        }
    }

    println!("Processed {} accounts before shutdown", counter);
    println!("Unsubscribing from account updates...");
    unsubscriber().await;
    println!("Shutdown complete");

    Ok(())
}

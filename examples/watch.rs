use dotenv::dotenv;
use futures_util::StreamExt;

use common_core::solana_pubsub_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let ws_url = dotenv::var("WS")?;
    println!("Connecting...");
    let pubsub_client = solana_pubsub_client(&ws_url).await?;
    println!("Connected...");

    let (mut accounts, unsubscriber) = pubsub_client.slot_subscribe().await?;

    println!("Wait...");
    while let Some(response) = accounts.next().await {
        dbg!(&response);
    }

    unsubscriber().await;

    Ok(())
}

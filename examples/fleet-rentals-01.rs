use anchor_lang::Discriminator;
use dotenv::dotenv;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};

use common_core::{solana_rpc_client, sqlite_pool};
use staratlas_fleet_rentals::{seeds, ID as FLEET_RENTALS_ID};
use staratlas_sage::{
    state::{Fleet, FleetShips, Ship},
    ID as SAGE_ID,
};

const UPSERT_FLEET_RENTALS_SQL: &str = r#"
    INSERT INTO staratlas_fleet_rentals_accounts(
        pubkey,
        lamports,
        data,
        owner,
        executable,
        rent_epoch
    ) VALUES (
        $1, $2, $3, $4, $5, $6
    ) ON CONFLICT (pubkey) DO UPDATE SET
        lamports = $2,
        data = $3,
        owner = $4,
        executable = $5,
        rent_epoch = $6
"#;

const UPSERT_SAGE_SQL: &str = r#"
    INSERT INTO staratlas_sage_accounts(
        pubkey,
        lamports,
        data,
        owner,
        executable,
        rent_epoch
    ) VALUES (
        $1, $2, $3, $4, $5, $6
    ) ON CONFLICT (pubkey) DO UPDATE SET
        lamports = $2,
        data = $3,
        owner = $4,
        executable = $5,
        rent_epoch = $6
"#;

async fn get_program_accounts(
    client: &RpcClient,
    program_id: &Pubkey,
    discrim: &[u8],
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        account_config: RpcAccountInfoConfig {
            encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
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
    dbg!(&accounts.len());

    Ok(accounts)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = sqlite_pool(&database_url).await?;

    let rpc_url = dotenv::var("RPC")?;
    let client = solana_rpc_client(&rpc_url).await?;

    let accounts = get_program_accounts(
        &client,
        &FLEET_RENTALS_ID,
        &seeds::CONTRACT_STATE_DISCRIMINATOR,
    )
    .await?;

    for (pubkey, account) in accounts {
        let _ = sqlx::query(UPSERT_FLEET_RENTALS_SQL)
            .bind(pubkey.to_string())
            .bind(account.lamports as i64)
            .bind(account.data)
            .bind(account.owner.to_string())
            .bind(account.executable)
            .bind(account.rent_epoch as i64)
            .execute(&pool)
            .await?;
    }

    // let _ = get_program_accounts(&client, &seeds::FLEET_DISCRIMINATOR)?;
    // let _ = get_program_accounts(&client, &seeds::RENTAL_STATE_DISCRIMINATOR)?;
    // let _ = get_program_accounts(&client, &seeds::RENTAL_DISCRIMINATOR)?;

    for discrim in [
        Fleet::DISCRIMINATOR,
        FleetShips::DISCRIMINATOR,
        Ship::DISCRIMINATOR,
    ] {
        let accounts = get_program_accounts(&client, &SAGE_ID, discrim).await?;
        for (pubkey, account) in accounts {
            let _ = sqlx::query(UPSERT_SAGE_SQL)
                .bind(pubkey.to_string())
                .bind(account.lamports as i64)
                .bind(account.data)
                .bind(account.owner.to_string())
                .bind(account.executable)
                .bind(account.rent_epoch as i64)
                .execute(&pool)
                .await?;
        }
    }

    Ok(())
}

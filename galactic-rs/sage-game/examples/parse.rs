use anchor_lang::{Discriminator, prelude::borsh::BorshDeserialize};
use dotenv::dotenv;

use common_core::{sqlite_pool, sqlx};
use staratlas_sage::{state::Fleet, ui::UiFleet};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = sqlite_pool(&database_url).await?;
    sqlx::query("DELETE FROM _sqlx_migrations")
        .execute(&pool)
        .await?;
    sqlx::migrate!("./../../migrations/sage").run(&pool).await?;

    let rows = sqlx::query!("SELECT * FROM staratlas_sage_accounts")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        match &row.data[..8] {
            Fleet::DISCRIMINATOR => {
                let mut data = row.data.as_slice();
                let fleet = UiFleet::deserialize_reader(&mut data)?;
                let data = serde_json::to_value(fleet)?;
                dbg!(&data);

                sqlx::query(
                    "INSERT OR REPLACE INTO sage_ui_fleets (pubkey, owner, data) VALUES (?, ?, ?)",
                )
                .bind(row.pubkey)
                .bind(row.owner)
                .bind(data)
                .execute(&pool)
                .await?;
            }
            _ => {}
        }
    }

    Ok(())
}

use anchor_lang::prelude::{AnchorDeserialize, Discriminator};
use dotenv::dotenv;
use sqlx::{
    Row,
    sqlite::{SqlitePool, SqlitePoolOptions},
};

use staratlas_fleet_rentals::{seeds, state::ContractState};
use staratlas_sage::{
    state::{Fleet, FleetShips, Ship},
    typedefs::FleetShipsInfo,
};

async fn upsert_fleet_rentals_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    const UPSERT_RENTAL_CONTRACT_STATES_SQL: &str = r#"
        INSERT INTO rental_contract_states (
            pubkey,
            fleet,
            rate,
            current_rental_state,
            owner_profile
        ) VALUES (
            $1, $2, $3, $4, $5
        ) ON CONFLICT (pubkey) DO UPDATE SET
            fleet = $2,
            rate = $3,
            current_rental_state = $4,
            owner_profile = $5
    "#;

    let rows = sqlx::query("SELECT pubkey, data FROM staratlas_fleet_rentals_accounts")
        .fetch_all(pool)
        .await?;

    for r in rows.iter() {
        let pubkey = r.get::<String, _>("pubkey");
        let mut data = r.get::<&[u8], _>("data");

        if data[..8] == seeds::CONTRACT_STATE_DISCRIMINATOR {
            data = &data[8..]; // Skip the first 8 bytes

            let contract = ContractState::try_from_slice(data)?;
            let res = sqlx::query(UPSERT_RENTAL_CONTRACT_STATES_SQL)
                .bind(pubkey)
                .bind(contract.fleet.to_string())
                .bind(contract.rate as i64)
                .bind(contract.current_rental_state.to_string())
                .bind(contract.owner_profile.to_string())
                .execute(pool)
                .await?;
            dbg!(res);
        }
    }

    Ok(())
}

async fn table_upsert_sage_fleet(
    pool: &SqlitePool,
    pubkey: &str,
    data: &[u8],
) -> anyhow::Result<()> {
    const UPSERT_SAGE_FLEETS_SQL: &str = r#"
        INSERT INTO sage_fleets (
            pubkey,
            game_id,
            owner_profile,
            fleet_ships,
            faction,
            fleet_label,
            ship_counts
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7
        ) ON CONFLICT (pubkey) DO UPDATE SET
            game_id = $2,
            owner_profile = $3,
            fleet_ships = $4,
            faction = $5,
            fleet_label = $6,
            ship_counts = $7
    "#;

    let mut data = &data[8..]; // Skip the first 8 bytes
    let fleet = Fleet::deserialize_reader(&mut data)?;
    let fleet_label = String::from_utf8_lossy(&fleet.fleet_label)
        .trim_end_matches('\0')
        .to_string();
    let ship_counts = serde_json::json!({
        "total": fleet.ship_counts.total,
        "updated": fleet.ship_counts.updated,
        "xx_small": fleet.ship_counts.xx_small,
        "x_small": fleet.ship_counts.x_small,
        "small": fleet.ship_counts.small,
        "medium": fleet.ship_counts.medium,
        "large": fleet.ship_counts.large,
        "capital": fleet.ship_counts.capital,
        "commander": fleet.ship_counts.commander,
        "titan": fleet.ship_counts.titan,
    });

    let res = sqlx::query(UPSERT_SAGE_FLEETS_SQL)
        .bind(pubkey)
        .bind(fleet.game_id.to_string())
        .bind(fleet.owner_profile.to_string())
        .bind(fleet.fleet_ships.to_string())
        .bind(fleet.faction)
        .bind(fleet_label)
        .bind(ship_counts)
        .execute(pool)
        .await?;
    dbg!(res);

    Ok(())
}

async fn table_upsert_sage_fleet_ships(
    pool: &SqlitePool,
    pubkey: &str,
    data: &[u8],
) -> anyhow::Result<()> {
    const UPSERT_FLEET_SHIPS_SQL: &str = r#"
            INSERT INTO sage_fleet_ships (
                pubkey,
                fleet,
                idx,
                ship,
                amount,
                fleet_ships_info_count
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT (pubkey, fleet, idx) DO UPDATE SET
                ship = $4,
                amount = $5,
                fleet_ships_info_count = $6
        "#;

    let mut data = &data[8..]; // Skip the first 8 bytes
    let fleet_ships = FleetShips::deserialize_reader(&mut data)?;

    // Start a transaction for the bulk insert
    let mut tx = pool.begin().await?;

    for idx in 0..fleet_ships.fleet_ships_info_count {
        let fleet_ships_info = FleetShipsInfo::deserialize_reader(&mut data)?;

        sqlx::query(UPSERT_FLEET_SHIPS_SQL)
            .bind(pubkey)
            .bind(fleet_ships.fleet.to_string())
            .bind(idx as i32)
            .bind(fleet_ships_info.ship.to_string())
            .bind(fleet_ships_info.amount as i64)
            .bind(fleet_ships.fleet_ships_info_count)
            .execute(&mut *tx)
            .await?;
    }

    // Commit the transaction
    let res = tx.commit().await?;
    dbg!(res, pubkey);

    assert_eq!(data.len(), 0);
    Ok(())
}

async fn table_upsert_sage_ships(
    pool: &SqlitePool,
    pubkey: &str,
    data: &[u8],
) -> anyhow::Result<()> {
    const UPSERT_SAGE_SHIPS_SQL: &str = r#"
        INSERT INTO sage_ships (
            pubkey,
            game_id,
            mint,
            name,
            size_class
        ) VALUES (
            $1, $2, $3, $4, $5
        ) ON CONFLICT (pubkey) DO UPDATE SET
            game_id = $2,
            mint = $3,
            name = $4,
            size_class = $5
    "#;

    let mut data = &data[8..]; // Skip the first 8 bytes
    let ship = Ship::deserialize_reader(&mut data)?;
    let name = String::from_utf8_lossy(&ship.name)
        .trim_end_matches('\0')
        .to_string();

    let res = sqlx::query(UPSERT_SAGE_SHIPS_SQL)
        .bind(pubkey)
        .bind(ship.game_id.to_string())
        .bind(ship.mint.to_string())
        .bind(name)
        .bind(ship.size_class)
        .execute(pool)
        .await?;
    dbg!(res);

    Ok(())
}

async fn upsert_sage_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    let rows = sqlx::query("SELECT pubkey, data FROM staratlas_sage_accounts")
        .fetch_all(pool)
        .await?;

    for r in rows.iter() {
        let pubkey = r.get::<String, _>("pubkey");
        let data = r.get::<&[u8], _>("data");

        match &data[..8] {
            Fleet::DISCRIMINATOR => table_upsert_sage_fleet(pool, &pubkey, data).await?,
            FleetShips::DISCRIMINATOR => table_upsert_sage_fleet_ships(pool, &pubkey, data).await?,
            Ship::DISCRIMINATOR => table_upsert_sage_ships(pool, &pubkey, data).await?,
            _ => continue,
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = SqlitePoolOptions::new().connect(&database_url).await?;

    upsert_fleet_rentals_tables(&pool).await?;
    upsert_sage_tables(&pool).await?;

    Ok(())
}

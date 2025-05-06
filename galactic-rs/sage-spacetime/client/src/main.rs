use spacetimedb_sdk::{DbContext, Error, Identity, Table, TableWithPrimaryKey, credentials};
use tokio::signal;

mod module_bindings;
use module_bindings::*;

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "sage-stdb";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = DbConnection::builder()
        .on_connect(|_ctx: &DbConnection, _identity: Identity, token: &str| {
            let _ = credentials::File::new(DB_NAME).save(token);
        })
        .on_connect_error(|_ctx: &ErrorContext, _error: Error| {
            std::process::exit(1);
        })
        .on_disconnect(|_ctx: &ErrorContext, _error: Option<Error>| {
            std::process::exit(0);
        })
        .with_token(credentials::File::new(DB_NAME).load()?)
        .with_module_name(DB_NAME)
        .with_uri(HOST)
        .build()?;

    ctx.subscription_builder()
        .on_applied(|ctx: &SubscriptionEventContext| {
            let stars = ctx.db.star().iter().collect::<Vec<_>>();
            dbg!(stars.len());

            let fleets = ctx.db.fleet().iter().collect::<Vec<_>>();
            dbg!(fleets.len());
        })
        .on_error(|_ctx: &ErrorContext, error: Error| {
            println!("Subscription error: {:?}", error);
        })
        .subscribe([
            "SELECT * FROM star",
            "SELECT * FROM fleet",
            "SELECT * FROM fleet_state",
            "SELECT * FROM fleet_pos",
        ]);

    // ctx.reducers
    //     .on_update_fleet(|_ctx: &ReducerEventContext, fleet: &SageFleet| {
    //         dbg!(fleet);
    //     });

    // ctx.reducers
    //     .on_update_fleet_state(|_ctx: &ReducerEventContext, state: &SageFleetState| {
    //         dbg!(state);
    //     });

    ctx.db
        .fleet_pos()
        .on_insert(|_ctx: &EventContext, pos: &SageFleetPos| {
            dbg!("INSERT", pos);
        });

    ctx.db.fleet_pos().on_update(
        |_ctx: &EventContext, old: &SageFleetPos, new: &SageFleetPos| {
            dbg!("UPDATE", old, new);
        },
    );

    loop {
        tokio::select! {
            // Process Websockets
            _ = ctx.run_async() => {}
            // Handel Ctrl+C signal
            _ = signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down gracefully...");
                break;
            }
        }
    }

    Ok(())
}

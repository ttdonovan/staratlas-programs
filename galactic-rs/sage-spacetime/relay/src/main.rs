use anchor_lang::{AccountDeserialize, AnchorDeserialize, Discriminator};
use dotenv::dotenv;
use futures_util::StreamExt;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use tokio::signal;

use common_core::{get_program_accounts, solana_pubsub_client, solana_rpc_client};
use staratlas_sage::{ID as SAGE_ID, state, ui};

use relay::{
    module_bindings::{self as stdb, *},
    spacetimedb_conn,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let ctx = spacetimedb_conn()?;

    let rpc_url = dotenv::var("RPC")?;
    let client = solana_rpc_client(&rpc_url).await?;

    let stars = get_program_accounts::<state::Star>(&client, &SAGE_ID).await?;
    for (key, account) in &stars {
        let mut bytes = account.data.as_slice();
        let star = state::Star::try_deserialize(&mut bytes)?;
        let star_name = String::from_utf8_lossy(&star.name)
            .trim_end_matches('\0')
            .to_string();

        let star = stdb::SageStar {
            pubkey: key.to_string(),
            name: star_name,
            sector_x: star.sector[0],
            sector_y: star.sector[1],
        };

        ctx.reducers.update_star(star)?;
    }

    let ws_url = dotenv::var("WS")?;
    let client = solana_pubsub_client(&ws_url).await?;

    let program_accounts_config = RpcProgramAccountsConfig {
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        ..Default::default()
    };

    let (mut accounts, unsubscriber) = client
        .program_subscribe(&SAGE_ID, Some(program_accounts_config))
        .await?;

    loop {
        tokio::select! {
            // Process Program account updates
            account_option = accounts.next() => {
                match account_option {
                    Some(response) => {
                        let pubkey = response.value.pubkey;
                        if let  Some(data) = response.value.account.data.decode() {
                            match &data[..8] {
                                state::Fleet::DISCRIMINATOR => {
                                    let mut bytes = data.as_slice();
                                    let fleet = ui::UiFleet::deserialize_reader(&mut bytes)?;
                                    let (state, pos) = match fleet.state {
                                        ui::UiFleetState::Idle(idle) => {
                                            let state = Some(stdb::FleetState::Idle(stdb::Idle {
                                                sector_x: idle.sector[0],
                                                sector_y: idle.sector[0],
                                            }));

                                            let pos = Some(stdb::SageFleetPos {
                                                pubkey: pubkey.clone(),
                                                x: idle.sector[0],
                                                y: idle.sector[0],
                                            });

                                            (state, pos)
                                        }
                                        ui::UiFleetState::MoveWarp(move_warp) => {
                                           let state = Some(stdb::FleetState::MoveWarp(stdb::MoveWarp {
                                               from_sector_x: move_warp.from_sector[0],
                                               from_sector_y: move_warp.from_sector[1],
                                               to_sector_x: move_warp.to_sector[0],
                                               to_sector_y: move_warp.to_sector[1],
                                           }));

                                           (state, None)
                                        }
                                        ui::UiFleetState::MoveSubwarp(move_subwarp) => {
                                            let state = Some(stdb::FleetState::MoveSubwarp(stdb::MoveSubwarp {
                                                from_sector_x: move_subwarp.from_sector[0],
                                                from_sector_y: move_subwarp.from_sector[1],
                                                to_sector_x: move_subwarp.to_sector[0],
                                                to_sector_y: move_subwarp.to_sector[1],
                                            }));

                                            (state, None)
                                        }
                                        _ => {
                                            // dbg!(fleet.state);
                                            (None, None)
                                        }
                                    };

                                    ctx.reducers.update_fleet(stdb::SageFleet {
                                        pubkey: pubkey.clone(),
                                        fleet_label: fleet.fleet_label,
                                        faction: fleet.faction,
                                    })?;

                                    if let Some(state) = state {
                                        ctx.reducers.update_fleet_state(stdb::SageFleetState {
                                            pubkey: pubkey,
                                            state,
                                        })?;
                                    }

                                    if let Some(pos) = pos {
                                        ctx.reducers.update_fleet_pos(pos)?;
                                    }
                                }
                                _ => {
                                    // println!("Skip: {:?}", pubkey);
                                }
                            }
                        };
                    }
                    None => {
                        println!("Account stream ended");
                        break;
                    }
                }
            }
            // Process Spacetimedb Websockets
            _ = ctx.run_async() => {}
            // Handel Ctrl+C signal
            _ = signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down gracefully...");
                break;
            }
        }
    }

    unsubscriber().await;

    Ok(())
}

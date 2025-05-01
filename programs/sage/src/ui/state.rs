use crate::{state, typedefs, ui::*};
use anchor_lang::prelude::borsh;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum UiFleetState {
    StarbaseLoadingBay(UiStarbaseLoadingBay),
    Idle(UiIdle),
    MineAsteroid(UiMineAsteroid),
    MoveWarp(UiMoveWarp),
    MoveSubwarp(UiMoveSubwarp),
    Respawn(UiRespawn),
}

impl borsh::BorshDeserialize for UiFleetState {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let discriminator = u8::deserialize_reader(reader)?;
        match discriminator {
            0 => Ok(UiFleetState::StarbaseLoadingBay(
                typedefs::StarbaseLoadingBay::deserialize_reader(reader)?.into(),
            )),
            1 => Ok(UiFleetState::Idle(
                typedefs::Idle::deserialize_reader(reader)?.into(),
            )),
            2 => Ok(UiFleetState::MineAsteroid(
                typedefs::MineAsteroid::deserialize_reader(reader)?.into(),
            )),
            3 => Ok(UiFleetState::MoveWarp(
                typedefs::MoveWarp::deserialize_reader(reader)?.into(),
            )),
            4 => Ok(UiFleetState::MoveSubwarp(
                typedefs::MoveSubwarp::deserialize_reader(reader)?.into(),
            )),
            5 => Ok(UiFleetState::Respawn(
                typedefs::Respawn::deserialize_reader(reader)?.into(),
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid FleetState discriminator: {}", discriminator),
            )),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiFleet {
    pub version: u8,
    pub game_id: String,
    pub owner_profile: String,
    pub fleet_ships: String,
    pub sub_profile: String,
    pub sub_profile_invalidator: String,
    pub faction: u8,
    pub fleet_label: String,
    pub ship_counts: UiShipCounts,
    pub warp_cooldown_expires_at: i64,
    pub scan_cooldown_expires_at: i64,
    pub stats: UiShipStats,
    pub cargo_hold: String,
    pub fuel_tank: String,
    pub ammo_bank: String,
    pub update_id: u64,
    pub state: UiFleetState,
}

impl borsh::BorshDeserialize for UiFleet {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        // Skip the 8-byte discriminator
        let mut discriminator = [0u8; 8];
        reader.read_exact(&mut discriminator)?;

        // Deserialize the Fleet state
        let account = state::Fleet::deserialize_reader(reader)?;

        // Deserialize the FleetState
        let state = UiFleetState::deserialize_reader(reader)?;

        let fleet_label = String::from_utf8_lossy(&account.fleet_label)
            .trim_end_matches('\0')
            .to_string();

        Ok(UiFleet {
            version: account.version,
            game_id: account.game_id.to_string(),
            owner_profile: account.owner_profile.to_string(),
            fleet_ships: account.fleet_ships.to_string(),
            sub_profile: account.sub_profile.key.to_string(),
            sub_profile_invalidator: account.sub_profile_invalidator.to_string(),
            faction: account.faction,
            fleet_label,
            ship_counts: account.ship_counts.into(),
            warp_cooldown_expires_at: account.warp_cooldown_expires_at,
            scan_cooldown_expires_at: account.scan_cooldown_expires_at,
            stats: account.stats.into(),
            cargo_hold: account.cargo_hold.to_string(),
            fuel_tank: account.fuel_tank.to_string(),
            ammo_bank: account.ammo_bank.to_string(),
            update_id: account.update_id,
            state,
        })
    }
}

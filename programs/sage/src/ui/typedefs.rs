use crate::typedefs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UiStarbaseLoadingBay {
    pub starbase: String,
    pub last_update: i64,
}

impl From<typedefs::StarbaseLoadingBay> for UiStarbaseLoadingBay {
    fn from(value: typedefs::StarbaseLoadingBay) -> Self {
        UiStarbaseLoadingBay {
            starbase: value.starbase.to_string(),
            last_update: value.last_update,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiIdle {
    pub sector: [i64; 2],
}

impl From<typedefs::Idle> for UiIdle {
    fn from(value: typedefs::Idle) -> Self {
        UiIdle {
            sector: value.sector,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiMineAsteroid {
    pub asteroid: String,
    pub resource: String,
    pub start: i64,
    pub end: i64,
    pub amount_mined: u64,
    pub last_update: i64,
}

impl From<typedefs::MineAsteroid> for UiMineAsteroid {
    fn from(value: typedefs::MineAsteroid) -> Self {
        UiMineAsteroid {
            asteroid: value.asteroid.to_string(),
            resource: value.resource.to_string(),
            start: value.start,
            end: value.end,
            amount_mined: value.amount_mined,
            last_update: value.last_update,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiMoveWarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub warp_start: i64,
    pub warp_finish: i64,
}

impl From<typedefs::MoveWarp> for UiMoveWarp {
    fn from(value: typedefs::MoveWarp) -> Self {
        UiMoveWarp {
            from_sector: value.from_sector,
            to_sector: value.to_sector,
            warp_start: value.warp_start,
            warp_finish: value.warp_finish,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiMoveSubwarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub current_sector: [i64; 2],
    pub departure_time: i64,
    pub arrival_time: i64,
    pub fuel_expenditure: u64,
    pub last_update: i64,
}

impl From<typedefs::MoveSubwarp> for UiMoveSubwarp {
    fn from(value: typedefs::MoveSubwarp) -> Self {
        UiMoveSubwarp {
            from_sector: value.from_sector,
            to_sector: value.to_sector,
            current_sector: value.current_sector,
            departure_time: value.departure_time,
            arrival_time: value.arrival_time,
            fuel_expenditure: value.fuel_expenditure,
            last_update: value.last_update,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiRespawn {
    pub sector: [i64; 2],
    pub start: i64,
}

impl From<typedefs::Respawn> for UiRespawn {
    fn from(value: typedefs::Respawn) -> Self {
        UiRespawn {
            sector: value.sector,
            start: value.start,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiShipCounts {
    pub total: u32,
    pub updated: u32,
    pub xx_small: u16,
    pub x_small: u16,
    pub small: u16,
    pub medium: u16,
    pub large: u16,
    pub capital: u16,
    pub commander: u16,
    pub titan: u16,
}

impl From<typedefs::ShipCounts> for UiShipCounts {
    fn from(value: typedefs::ShipCounts) -> Self {
        UiShipCounts {
            total: value.total,
            updated: value.updated,
            xx_small: value.xx_small,
            x_small: value.x_small,
            small: value.small,
            medium: value.medium,
            large: value.large,
            capital: value.capital,
            commander: value.commander,
            titan: value.titan,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiShipStats {
    pub movement_stats: UiMovementStats,
    pub cargo_stats: UiCargoStats,
    pub misc_stats: UiMiscStats,
}

impl From<typedefs::ShipStats> for UiShipStats {
    fn from(value: typedefs::ShipStats) -> Self {
        UiShipStats {
            movement_stats: value.movement_stats.into(),
            cargo_stats: value.cargo_stats.into(),
            misc_stats: value.misc_stats.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiMovementStats {
    pub subwarp_speed: u32,
    pub warp_speed: u32,
    pub max_warp_distance: u16,
    pub warp_cool_down: u16,
    pub subwarp_fuel_consumption_rate: u32,
    pub warp_fuel_consumption_rate: u32,
    pub planet_exit_fuel_amount: u32,
}

impl From<typedefs::MovementStats> for UiMovementStats {
    fn from(value: typedefs::MovementStats) -> Self {
        UiMovementStats {
            subwarp_speed: value.subwarp_speed,
            warp_speed: value.warp_speed,
            max_warp_distance: value.max_warp_distance,
            warp_cool_down: value.warp_cool_down,
            subwarp_fuel_consumption_rate: value.subwarp_fuel_consumption_rate,
            warp_fuel_consumption_rate: value.warp_fuel_consumption_rate,
            planet_exit_fuel_amount: value.planet_exit_fuel_amount,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiCargoStats {
    pub cargo_capacity: u32,
    pub fuel_capacity: u32,
    pub ammo_capacity: u32,
    pub ammo_consumption_rate: u32,
    pub food_consumption_rate: u32,
    pub mining_rate: u32,
    pub upgrade_rate: u32,
    pub cargo_transfer_rate: u32,
    pub tractor_beam_gather_rate: u32,
}

impl From<typedefs::CargoStats> for UiCargoStats {
    fn from(value: typedefs::CargoStats) -> Self {
        UiCargoStats {
            cargo_capacity: value.cargo_capacity,
            fuel_capacity: value.fuel_capacity,
            ammo_capacity: value.ammo_capacity,
            ammo_consumption_rate: value.ammo_consumption_rate,
            food_consumption_rate: value.food_consumption_rate,
            mining_rate: value.mining_rate,
            upgrade_rate: value.upgrade_rate,
            cargo_transfer_rate: value.cargo_transfer_rate,
            tractor_beam_gather_rate: value.tractor_beam_gather_rate,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UiMiscStats {
    pub required_crew: u16,
    pub passenger_capacity: u16,
    pub crew_count: u16,
    pub rented_crew: u16,
    pub respawn_time: u16,
    pub scan_cool_down: u16,
    pub sdu_per_scan: u32,
    pub scan_cost: u32,
    pub placeholder: u32,
    pub placeholder2: u32,
    pub placeholder3: u32,
}

impl From<typedefs::MiscStats> for UiMiscStats {
    fn from(value: typedefs::MiscStats) -> Self {
        UiMiscStats {
            required_crew: value.required_crew,
            passenger_capacity: value.passenger_capacity,
            crew_count: value.crew_count,
            rented_crew: value.rented_crew,
            respawn_time: value.respawn_time,
            scan_cool_down: value.scan_cool_down,
            sdu_per_scan: value.sdu_per_scan,
            scan_cost: value.scan_cost,
            placeholder: value.placeholder,
            placeholder2: value.placeholder2,
            placeholder3: value.placeholder3,
        }
    }
}

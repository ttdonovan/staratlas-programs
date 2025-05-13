use spacetimedb::{reducer, table, ReducerContext, SpacetimeType, Table};

#[table(name = fleet, public)]
pub struct SageFleet {
    #[primary_key]
    pub pubkey: String,
    pub fleet_label: String,
    pub faction: u8,
}

#[derive(SpacetimeType)]
pub struct Idle {
    pub sector_x: i64,
    pub sector_y: i64,
}

#[derive(SpacetimeType)]
pub struct MoveWarp {
    pub from_sector_x: i64,
    pub from_sector_y: i64,
    pub to_sector_x: i64,
    pub to_sector_y: i64,
    pub warp_start: i64,
    pub warp_finish: i64,
}

#[derive(SpacetimeType)]
pub struct MoveSubwarp {
    pub from_sector_x: i64,
    pub from_sector_y: i64,
    pub to_sector_x: i64,
    pub to_sector_y: i64,
    pub depature_time: i64,
    pub arrival_time: i64,
}

#[derive(SpacetimeType)]
pub enum FleetState {
    Idle(Idle),
    MoveWarp(MoveWarp),
    MoveSubwarp(MoveSubwarp),
}

#[table(name = fleet_state, public)]
pub struct SageFleetState {
    #[primary_key]
    pub pubkey: String,
    pub state: FleetState,
}

#[table(name = fleet_warping, public)]
pub struct SageFleetWarping {
    #[primary_key]
    pub pubkey: String,
    pub from_sector_x: i64,
    pub from_sector_y: i64,
    pub to_sector_x: i64,
    pub to_sector_y: i64,
    pub slot_start: u64,
    pub slot_finish: u64,
}

#[table(name = fleet_pos, public)]
pub struct SageFleetPos {
    #[primary_key]
    pub pubkey: String,
    pub x: i64,
    pub y: i64,
}

#[reducer]
pub fn update_fleet(ctx: &ReducerContext, fleet: SageFleet) {
    if let Some(_found) = ctx.db.fleet().pubkey().find(&fleet.pubkey) {
        ctx.db.fleet().pubkey().update(fleet);
    } else {
        ctx.db.fleet().insert(fleet);
    }
}

#[reducer]
pub fn update_fleet_state(ctx: &ReducerContext, state: SageFleetState) {
    if let Some(found) = ctx.db.fleet_state().pubkey().find(&state.pubkey) {
        match &found.state {
            FleetState::MoveWarp(_) => {
                ctx.db.fleet_warping().pubkey().delete(&state.pubkey);
            }
            FleetState::MoveSubwarp(_) => {
                ctx.db.fleet_warping().pubkey().delete(&state.pubkey);
            }
            _ => {}
        }

        match &state.state {
            FleetState::Idle(idle) => {
                update_fleet_pos(
                    ctx,
                    SageFleetPos {
                        pubkey: state.pubkey.clone(),
                        x: idle.sector_x,
                        y: idle.sector_y,
                    },
                );
            }
            FleetState::MoveWarp(move_warp) => {
                ctx.db.fleet_pos().pubkey().delete(&state.pubkey);
                ctx.db.fleet_warping().insert(SageFleetWarping {
                    pubkey: state.pubkey.clone(),
                    from_sector_x: move_warp.from_sector_x,
                    from_sector_y: move_warp.from_sector_y,
                    to_sector_x: move_warp.to_sector_x,
                    to_sector_y: move_warp.to_sector_y,
                    slot_start: move_warp.warp_start as u64,
                    slot_finish: move_warp.warp_finish as u64,
                });
            }
            FleetState::MoveSubwarp(move_subwarp) => {
                ctx.db.fleet_pos().pubkey().delete(&state.pubkey);
                ctx.db.fleet_warping().insert(SageFleetWarping {
                    pubkey: state.pubkey.clone(),
                    from_sector_x: move_subwarp.from_sector_x,
                    from_sector_y: move_subwarp.from_sector_y,
                    to_sector_x: move_subwarp.to_sector_x,
                    to_sector_y: move_subwarp.to_sector_y,
                    slot_start: move_subwarp.depature_time as u64,
                    slot_finish: move_subwarp.arrival_time as u64,
                });
            }
            _ => {}
        }

        ctx.db.fleet_state().pubkey().update(state);
    } else {
        ctx.db.fleet_state().insert(state);
    }
}

#[reducer]
pub fn update_fleet_pos(ctx: &ReducerContext, pos: SageFleetPos) {
    if let Some(_found) = ctx.db.fleet_pos().pubkey().find(&pos.pubkey) {
        ctx.db.fleet_pos().pubkey().update(pos);
    } else {
        ctx.db.fleet_pos().insert(pos);
    }
}

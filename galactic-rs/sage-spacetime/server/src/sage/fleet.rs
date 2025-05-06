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
}

#[derive(SpacetimeType)]
pub struct MoveSubwarp {
    pub from_sector_x: i64,
    pub from_sector_y: i64,
    pub to_sector_x: i64,
    pub to_sector_y: i64,
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
    if let Some(_found) = ctx.db.fleet_state().pubkey().find(&state.pubkey) {
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

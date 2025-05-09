use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

#[table(name = player_viewport, public)]
pub struct PlayerViewport {
    #[primary_key]
    id: Identity,
    x: i32,
    y: i32,
    size: i32,
}

#[reducer]
pub fn set_player_viewport(ctx: &ReducerContext, x: i32, y: i32, size: i32) {
    if let Some(viewport) = ctx.db.player_viewport().id().find(ctx.sender) {
        ctx.db.player_viewport().id().update(PlayerViewport {
            x,
            y,
            size,
            ..viewport
        });
    } else {
        ctx.db.player_viewport().insert(PlayerViewport {
            id: ctx.sender,
            x,
            y,
            size,
        });
    }
}

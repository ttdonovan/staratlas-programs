use spacetimedb::{
    client_visibility_filter, reducer, table, Filter, Identity, ReducerContext, Table,
};

// #[client_visibility_filter]
// const PLAYER_FLEET_POS_FILTER: Filter = Filter::Sql(
//     "SELECT p.*
//     FROM fleet_pos p
//     JOIN player_viewport v
//     WHERE v.id = :sender AND
//         (p.x > v.tl_x AND
//         p.y > v.tl_y AND
//         p.x < v.br_x AND
//         p.y < v.br_y)",
// );

#[table(name = player_viewport, public)]
pub struct PlayerViewport {
    #[primary_key]
    id: Identity,
    tl_x: i64,
    tl_y: i64,
    br_x: i64,
    br_y: i64,
}

#[reducer]
pub fn set_player_viewport(ctx: &ReducerContext, x: i64, y: i64, size: i64, _margin: i64) {
    let tl_x = x;
    let tl_y = y;
    let br_x = x + size + y;
    let br_y = y + size;

    if let Some(viewport) = ctx.db.player_viewport().id().find(ctx.sender) {
        ctx.db.player_viewport().id().update(PlayerViewport {
            tl_x,
            tl_y,
            br_x,
            br_y,
            ..viewport
        });
    } else {
        ctx.db.player_viewport().insert(PlayerViewport {
            id: ctx.sender,
            tl_x,
            tl_y,
            br_x,
            br_y,
        });
    }
}

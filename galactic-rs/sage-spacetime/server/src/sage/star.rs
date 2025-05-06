use spacetimedb::{reducer, table, ReducerContext, Table};

#[table(name = star, public)]
pub struct SageStar {
    #[primary_key]
    pub pubkey: String,
    pub name: String,
    pub sector_x: i64,
    pub sector_y: i64,
}

#[reducer]
pub fn update_star(ctx: &ReducerContext, star: SageStar) {
    if let Some(_found) = ctx.db.star().pubkey().find(&star.pubkey) {
        ctx.db.star().pubkey().update(star);
    } else {
        ctx.db.star().insert(star);
    }
}

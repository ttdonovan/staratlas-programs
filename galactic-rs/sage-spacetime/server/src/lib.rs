use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

pub mod player;
pub mod sage;

#[spacetimedb::table(name = config, public)]
pub struct Config {
    #[primary_key]
    pub id: u32,
    pub slot: u64,
}

#[table(name = client, public)]
#[table(name = logged_out_client)]
pub struct Client {
    #[primary_key]
    id: Identity,
}

#[table(name = message, public)]
pub struct Message {
    sender: Identity,
    text: String,
    sent: Timestamp,
}

#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let sender = ctx.sender;
    log::info!("[{}]: {}", &sender, &text);

    ctx.db.message().insert(Message {
        sender,
        text,
        sent: ctx.timestamp,
    });

    Ok(())
}

#[reducer]
pub fn debug(ctx: &ReducerContext) {
    log::info!("This reducer was called by: {}", ctx.sender);
}

#[reducer]
pub fn update_config_slot(ctx: &ReducerContext, slot: u64) {
    if let Some(mut config) = ctx.db.config().id().find(0) {
        config.slot = slot;
        ctx.db.config().id().update(config);
    }
}

#[reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    // Called when the module is initially published
    ctx.db.config().try_insert(Config { id: 0, slot: 0 })?;
    Ok(())
}

#[reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    if let Some(client) = ctx.db.client().id().find(ctx.sender) {
        log::info!("Existing client connected: {}", client.id);
    } else {
        log::info!("New client connected: {}", ctx.sender);
        ctx.db.client().insert(Client { id: ctx.sender });
    }
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {
    // Called everytime a client disconnects
}

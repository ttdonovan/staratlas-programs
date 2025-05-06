use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

pub mod sage;

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

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
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

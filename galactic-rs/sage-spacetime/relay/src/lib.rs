use spacetimedb_sdk::{Error, Identity, credentials};

pub mod module_bindings;
use module_bindings::*;

const SPACETIMEDB_HOST: &str = "http://localhost:3000";
const SPACETIMEDB_NAME: &str = "sage-stdb";

pub fn spacetimedb_conn() -> anyhow::Result<DbConnection> {
    let ctx = DbConnection::builder()
        .on_connect(|_ctx: &DbConnection, _identity: Identity, token: &str| {
            let _ = credentials::File::new(SPACETIMEDB_NAME).save(token);
        })
        .on_connect_error(|_ctx: &ErrorContext, _error: Error| {
            panic!();
        })
        .on_disconnect(|_ctx: &ErrorContext, error: Option<Error>| {
            if error.is_some() {
                panic!();
            }
        })
        .with_token(credentials::File::new(SPACETIMEDB_NAME).load()?)
        .with_module_name(SPACETIMEDB_NAME)
        .with_uri(SPACETIMEDB_HOST)
        .build()?;

    Ok(ctx)
}

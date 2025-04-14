use spacetimedb::{reducer, Identity, ReducerContext, Table};

#[spacetimedb::table(name = user, public)]
pub struct User {
    #[primary_key]
    id: Identity,
    status: String,
}

#[reducer]
pub fn set_status(ctx: &ReducerContext, content: String) -> Result<(), String> {
    if let Some(user) = ctx.db.user().id().find(ctx.sender) {
        ctx.db.user().id().update(User {
            status: content,
            ..user
        });
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().id().find(ctx.sender) {
        ctx.db.user().id().update(User {
            status: "Welcome back!".to_string(),
            ..user
        });
    } else {
        ctx.db.user().insert(User {
            id: ctx.sender,
            status: "Welcome!".to_string(),
        });
    }
}

#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().id().find(ctx.sender) {
        ctx.db.user().id().update(User {
            status: "Come back soon!".to_string(),
            ..user
        });
    } else {
        log::warn!("Disconnect event for unknown user: {:?}", ctx.sender);
    }
}

use std::rc::Rc;

use db_binding::*;
use godot::prelude::*;
use spacetimedb_sdk::{
    DbContext, Error, Identity, Status, Table, TableWithPrimaryKey, credentials,
};

mod db_binding;

struct SpacetimeGD;

#[gdextension]
unsafe impl ExtensionLibrary for SpacetimeGD {}

#[derive(GodotClass)]
#[class(base=Node)]
struct SpacetimeDbClient {
    base: Base<Node>,
    db: Option<Rc<DbConnection>>,
}

#[godot_api]
impl INode for SpacetimeDbClient {
    fn init(base: Base<Node>) -> Self {
        Self { base, db: None }
    }

    fn ready(&mut self) {
        if !self.connect_to_db() {
            return godot_error!("Failed to connect to spacetimedb...");
        };

        if let Some(ctx) = self.db.as_ref() {
            Self::register_callbacks(ctx);
        }
    }

    fn physics_process(&mut self, _delta: f32) {
        let ctx = self.db.as_ref().expect("Database context...");
        if let Err(err) = ctx.frame_tick() {
            // Using `DbConnection::frame_tick()` to perform spacetime processing on main thread.
            godot_error!("{:?}", err);
        }
    }
}

#[godot_api]
impl SpacetimeDbClient {
    fn connect_to_db(&mut self) -> bool {
        match DbConnection::builder()
            .on_connect(Self::on_connected)
            .on_connect_error(Self::on_connect_error)
            .on_disconnect(Self::on_disconnected)
            .with_token(Self::creds_store().load().expect("Credentials..."))
            .with_module_name("spacetime-gd")
            .with_uri("http://localhost:3000")
            .build()
        {
            Ok(conn) => {
                self.db = Some(Rc::new(conn));
            }
            Err(err) => {
                godot_error!("{:?}", err);
                return false;
            }
        }

        if let Some(ctx) = self.db.clone() {
            ctx.subscription_builder()
                .on_applied(Self::on_sub_applied)
                .on_error(Self::on_sub_error)
                .subscribe(["select * from user"]);
        }

        true
    }

    fn creds_store() -> credentials::File {
        credentials::File::new("spacetime-gd")
    }

    fn on_connected(_ctx: &DbConnection, _id: Identity, token: &str) {
        if let Err(e) = Self::creds_store().save(token) {
            eprintln!("Failed to save credentials: {:?}", e);
        }
    }

    fn on_connect_error(_ctx: &ErrorContext, err: Error) {
        eprintln!("Connection error: {:?}", err);
    }

    fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
        if let Some(err) = err {
            eprintln!("Disconnected: {}", err);
        } else {
            println!("Lost connection to database.");
        }
    }

    fn register_callbacks(ctx: &DbConnection) {
        ctx.db.user().on_insert(Self::on_user_insert);
        ctx.db.user().on_update(Self::on_user_update);
        ctx.reducers.on_set_status(Self::on_status_set);
    }

    fn on_user_insert(_ctx: &EventContext, user: &User) {
        godot_print!("User connected with id: {}", user.id.to_hex().to_string())
    }

    fn on_user_update(_ctx: &EventContext, old: &User, new: &User) {
        if old.status != new.status {
            godot_print!(
                "User status changed: \"{}\" to -> \"{}\"",
                old.status,
                new.status
            )
        }
    }

    fn on_status_set(ctx: &ReducerEventContext, status: &String) {
        if let Status::Failed(err) = &ctx.event.status {
            godot_error!("Failed to change status to {:?}: {}", status, err);
        } else {
            godot_print!("Status updated: {}", status);
        }
    }

    fn on_sub_applied(ctx: &SubscriptionEventContext) {
        godot_print!("Getting connected users:");
        ctx.db.user().iter().for_each(|u| godot_print!("\n{:?}", u));
        godot_print!("\nSubscription applied.");
    }

    fn on_sub_error(_ctx: &ErrorContext, err: Error) {
        godot_error!("Subscription failed: {}", err);
    }

    #[func]
    fn update_status(&self, content: String) {
        let ctx = self.db.as_ref().expect("Database connection");

        if let Err(err) = ctx.reducers.set_status(content) {
            godot_error!("{:?}", err)
        }
    }
}

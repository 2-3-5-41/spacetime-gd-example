// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

pub mod client_connected_reducer;
pub mod client_disconnected_reducer;
pub mod set_status_reducer;
pub mod user_table;
pub mod user_type;

pub use client_connected_reducer::{
    client_connected, set_flags_for_client_connected, ClientConnectedCallbackId,
};
pub use client_disconnected_reducer::{
    client_disconnected, set_flags_for_client_disconnected, ClientDisconnectedCallbackId,
};
pub use set_status_reducer::{set_flags_for_set_status, set_status, SetStatusCallbackId};
pub use user_table::*;
pub use user_type::User;

#[derive(Clone, PartialEq, Debug)]

/// One of the reducers defined by this module.
///
/// Contained within a [`__sdk::ReducerEvent`] in [`EventContext`]s for reducer events
/// to indicate which reducer caused the event.

pub enum Reducer {
    ClientConnected,
    ClientDisconnected,
    SetStatus { content: String },
}

impl __sdk::InModule for Reducer {
    type Module = RemoteModule;
}

impl __sdk::Reducer for Reducer {
    fn reducer_name(&self) -> &'static str {
        match self {
            Reducer::ClientConnected => "client_connected",
            Reducer::ClientDisconnected => "client_disconnected",
            Reducer::SetStatus { .. } => "set_status",
        }
    }
}
impl TryFrom<__ws::ReducerCallInfo<__ws::BsatnFormat>> for Reducer {
    type Error = __sdk::Error;
    fn try_from(value: __ws::ReducerCallInfo<__ws::BsatnFormat>) -> __sdk::Result<Self> {
        match &value.reducer_name[..] {
            "client_connected" => Ok(__sdk::parse_reducer_args::<
                client_connected_reducer::ClientConnectedArgs,
            >("client_connected", &value.args)?
            .into()),
            "client_disconnected" => Ok(__sdk::parse_reducer_args::<
                client_disconnected_reducer::ClientDisconnectedArgs,
            >("client_disconnected", &value.args)?
            .into()),
            "set_status" => Ok(
                __sdk::parse_reducer_args::<set_status_reducer::SetStatusArgs>(
                    "set_status",
                    &value.args,
                )?
                .into(),
            ),
            unknown => {
                Err(
                    __sdk::InternalError::unknown_name("reducer", unknown, "ReducerCallInfo")
                        .into(),
                )
            }
        }
    }
}

#[derive(Default)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct DbUpdate {
    user: __sdk::TableUpdate<User>,
}

impl TryFrom<__ws::DatabaseUpdate<__ws::BsatnFormat>> for DbUpdate {
    type Error = __sdk::Error;
    fn try_from(raw: __ws::DatabaseUpdate<__ws::BsatnFormat>) -> Result<Self, Self::Error> {
        let mut db_update = DbUpdate::default();
        for table_update in raw.tables {
            match &table_update.table_name[..] {
                "user" => db_update.user = user_table::parse_table_update(table_update)?,

                unknown => {
                    return Err(__sdk::InternalError::unknown_name(
                        "table",
                        unknown,
                        "DatabaseUpdate",
                    )
                    .into());
                }
            }
        }
        Ok(db_update)
    }
}

impl __sdk::InModule for DbUpdate {
    type Module = RemoteModule;
}

impl __sdk::DbUpdate for DbUpdate {
    fn apply_to_client_cache(
        &self,
        cache: &mut __sdk::ClientCache<RemoteModule>,
    ) -> AppliedDiff<'_> {
        let mut diff = AppliedDiff::default();

        diff.user = cache
            .apply_diff_to_table::<User>("user", &self.user)
            .with_updates_by_pk(|row| &row.id);

        diff
    }
}

#[derive(Default)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct AppliedDiff<'r> {
    user: __sdk::TableAppliedDiff<'r, User>,
}

impl __sdk::InModule for AppliedDiff<'_> {
    type Module = RemoteModule;
}

impl<'r> __sdk::AppliedDiff<'r> for AppliedDiff<'r> {
    fn invoke_row_callbacks(
        &self,
        event: &EventContext,
        callbacks: &mut __sdk::DbCallbacks<RemoteModule>,
    ) {
        callbacks.invoke_table_row_callbacks::<User>("user", &self.user, event);
    }
}

#[doc(hidden)]
pub struct RemoteModule;

impl __sdk::InModule for RemoteModule {
    type Module = Self;
}

/// The `reducers` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
pub struct RemoteReducers {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for RemoteReducers {
    type Module = RemoteModule;
}

#[doc(hidden)]
/// The `set_reducer_flags` field of [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
/// Each method sets the flags for the reducer with the same name.
///
/// This type is currently unstable and may be removed without a major version bump.
pub struct SetReducerFlags {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for SetReducerFlags {
    type Module = RemoteModule;
}

/// The `db` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each table defined by the module.
pub struct RemoteTables {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for RemoteTables {
    type Module = RemoteModule;
}

/// A connection to a remote module, including a materialized view of a subset of the database.
///
/// Connect to a remote module by calling [`DbConnection::builder`]
/// and using the [`__sdk::DbConnectionBuilder`] builder-pattern constructor.
///
/// You must explicitly advance the connection by calling any one of:
///
/// - [`DbConnection::frame_tick`].
/// - [`DbConnection::run_threaded`].
/// - [`DbConnection::run_async`].
/// - [`DbConnection::advance_one_message`].
/// - [`DbConnection::advance_one_message_blocking`].
/// - [`DbConnection::advance_one_message_async`].
///
/// Which of these methods you should call depends on the specific needs of your application,
/// but you must call one of them, or else the connection will never progress.
pub struct DbConnection {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    #[doc(hidden)]
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,

    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for DbConnection {
    type Module = RemoteModule;
}

impl __sdk::DbContext for DbConnection {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl DbConnection {
    /// Builder-pattern constructor for a connection to a remote module.
    ///
    /// See [`__sdk::DbConnectionBuilder`] for required and optional configuration for the new connection.
    pub fn builder() -> __sdk::DbConnectionBuilder<RemoteModule> {
        __sdk::DbConnectionBuilder::new()
    }

    /// If any WebSocket messages are waiting, process one of them.
    ///
    /// Returns `true` if a message was processed, or `false` if the queue is empty.
    /// Callers should invoke this message in a loop until it returns `false`
    /// or for as much time is available to process messages.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::frame_tick`] each frame
    /// to fully exhaust the queue whenever time is available.
    pub fn advance_one_message(&self) -> __sdk::Result<bool> {
        self.imp.advance_one_message()
    }

    /// Process one WebSocket message, potentially blocking the current thread until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_threaded`] to spawn a thread
    /// which advances the connection automatically.
    pub fn advance_one_message_blocking(&self) -> __sdk::Result<()> {
        self.imp.advance_one_message_blocking()
    }

    /// Process one WebSocket message, `await`ing until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_async`] to run an `async` loop
    /// which advances the connection when polled.
    pub async fn advance_one_message_async(&self) -> __sdk::Result<()> {
        self.imp.advance_one_message_async().await
    }

    /// Process all WebSocket messages waiting in the queue,
    /// then return without `await`ing or blocking the current thread.
    pub fn frame_tick(&self) -> __sdk::Result<()> {
        self.imp.frame_tick()
    }

    /// Spawn a thread which processes WebSocket messages as they are received.
    pub fn run_threaded(&self) -> std::thread::JoinHandle<()> {
        self.imp.run_threaded()
    }

    /// Run an `async` loop which processes WebSocket messages when polled.
    pub async fn run_async(&self) -> __sdk::Result<()> {
        self.imp.run_async().await
    }
}

impl __sdk::DbConnection for DbConnection {
    fn new(imp: __sdk::DbContextImpl<RemoteModule>) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            imp,
        }
    }
}

/// A handle on a subscribed query.
// TODO: Document this better after implementing the new subscription API.
#[derive(Clone)]
pub struct SubscriptionHandle {
    imp: __sdk::SubscriptionHandleImpl<RemoteModule>,
}

impl __sdk::InModule for SubscriptionHandle {
    type Module = RemoteModule;
}

impl __sdk::SubscriptionHandle for SubscriptionHandle {
    fn new(imp: __sdk::SubscriptionHandleImpl<RemoteModule>) -> Self {
        Self { imp }
    }

    /// Returns true if this subscription has been terminated due to an unsubscribe call or an error.
    fn is_ended(&self) -> bool {
        self.imp.is_ended()
    }

    /// Returns true if this subscription has been applied and has not yet been unsubscribed.
    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    /// Unsubscribe from the query controlled by this `SubscriptionHandle`,
    /// then run `on_end` when its rows are removed from the client cache.
    fn unsubscribe_then(self, on_end: __sdk::OnEndedCallback<RemoteModule>) -> __sdk::Result<()> {
        self.imp.unsubscribe_then(Some(on_end))
    }

    fn unsubscribe(self) -> __sdk::Result<()> {
        self.imp.unsubscribe_then(None)
    }
}

/// Alias trait for a [`__sdk::DbContext`] connected to this module,
/// with that trait's associated types bounded to this module's concrete types.
///
/// Users can use this trait as a boundary on definitions which should accept
/// either a [`DbConnection`] or an [`EventContext`] and operate on either.
pub trait RemoteDbContext:
    __sdk::DbContext<
    DbView = RemoteTables,
    Reducers = RemoteReducers,
    SetReducerFlags = SetReducerFlags,
    SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
>
{
}
impl<
        Ctx: __sdk::DbContext<
            DbView = RemoteTables,
            Reducers = RemoteReducers,
            SetReducerFlags = SetReducerFlags,
            SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
        >,
    > RemoteDbContext for Ctx
{
}

/// An [`__sdk::DbContext`] augmented with a [`__sdk::Event`],
/// passed to [`__sdk::Table::on_insert`], [`__sdk::Table::on_delete`] and [`__sdk::TableWithPrimaryKey::on_update`] callbacks.
pub struct EventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: __sdk::Event<Reducer>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for EventContext {
    type Event = __sdk::Event<Reducer>;
    fn event(&self) -> &Self::Event {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

impl __sdk::InModule for EventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for EventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::EventContext for EventContext {}

/// An [`__sdk::DbContext`] augmented with a [`__sdk::ReducerEvent`],
/// passed to on-reducer callbacks.
pub struct ReducerEventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: __sdk::ReducerEvent<Reducer>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for ReducerEventContext {
    type Event = __sdk::ReducerEvent<Reducer>;
    fn event(&self) -> &Self::Event {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

impl __sdk::InModule for ReducerEventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for ReducerEventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::ReducerEventContext for ReducerEventContext {}

/// An [`__sdk::DbContext`] passed to [`__sdk::SubscriptionBuilder::on_applied`] and [`SubscriptionHandle::unsubscribe_then`] callbacks.
pub struct SubscriptionEventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for SubscriptionEventContext {
    type Event = ();
    fn event(&self) -> &Self::Event {
        &()
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, _event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            imp,
        }
    }
}

impl __sdk::InModule for SubscriptionEventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for SubscriptionEventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::SubscriptionEventContext for SubscriptionEventContext {}

/// An [`__sdk::DbContext`] augmented with a [`__sdk::Error`],
/// passed to [`__sdk::DbConnectionBuilder::on_disconnect`], [`__sdk::DbConnectionBuilder::on_connect_error`] and [`__sdk::SubscriptionBuilder::on_error`] callbacks.
pub struct ErrorContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: Option<__sdk::Error>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for ErrorContext {
    type Event = Option<__sdk::Error>;
    fn event(&self) -> &Self::Event {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

impl __sdk::InModule for ErrorContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for ErrorContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::ErrorContext for ErrorContext {}

impl __sdk::SpacetimeModule for RemoteModule {
    type DbConnection = DbConnection;
    type EventContext = EventContext;
    type ReducerEventContext = ReducerEventContext;
    type SubscriptionEventContext = SubscriptionEventContext;
    type ErrorContext = ErrorContext;
    type Reducer = Reducer;
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;
    type DbUpdate = DbUpdate;
    type AppliedDiff<'r> = AppliedDiff<'r>;
    type SubscriptionHandle = SubscriptionHandle;

    fn register_tables(client_cache: &mut __sdk::ClientCache<Self>) {
        user_table::register_table(client_cache);
    }
}

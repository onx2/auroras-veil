pub mod reducers;
pub mod subscription;

use crate::stdb::{
    CharacterDefTableAccess, CharacterPawnTableAccess, DbConnection, EntityMovementTableAccess,
    EntityTableAccess, RemoteTables, TransformTableAccess,
};
use bevy::prelude::*;
use bevy_spacetimedb::{ReadStdbConnectedMessage, StdbConnection, StdbPlugin};
use reducers::*;
pub use subscription::{StdbSubscriptions, SubKey};

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(StdbSubscriptions::default());
    app.add_plugins(
        StdbPlugin::default()
            .with_uri("http://127.0.0.1:3000")
            .with_module_name("av")
            // --------------------------------
            // Register all reducers
            // --------------------------------
            .add_reducer::<CreateCharacter>()
            .add_reducer::<DeleteCharacter>()
            .add_reducer::<EnterWorld>()
            .add_reducer::<LeaveWorld>()
            // --------------------------------
            // Register all tables
            // --------------------------------
            .add_table(RemoteTables::character_def)
            .add_table(RemoteTables::transform)
            .add_table(RemoteTables::entity)
            .add_table(RemoteTables::character_pawn)
            .add_table(RemoteTables::entity_movement)
            .with_run_fn(DbConnection::run_threaded),
    );

    app.add_systems(Update, on_connect);
}

fn on_connect(
    mut messages: ReadStdbConnectedMessage,
    stdb: SpacetimeDB,
    mut stdb_subscriptions: ResMut<StdbSubscriptions>,
) {
    for message in messages.read() {
        println!("SpacetimeDB module connected: {:?}", message.identity);
        stdb_subscriptions.upsert(
            SubKey::GlobalData,
            stdb.subscription_builder().subscribe(vec![
                "SELECT * FROM race",
                "SELECT * FROM class",
                "SELECT * FROM xp_progression",
            ]),
        );
    }
}

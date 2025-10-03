use crate::stdb::{DbConnection, Reducer, RemoteModule, RemoteReducers, add_reducer::add};
use bevy::prelude::*;
use bevy_spacetimedb::{
    ReadStdbConnectedMessage, RegisterReducerMessage, StdbConnection, StdbPlugin,
};
use spacetimedb_sdk::ReducerEvent;

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        StdbPlugin::default()
            .with_uri("http://127.0.0.1:3000")
            .with_module_name("av")
            .add_reducer::<Add>()
            .with_run_fn(DbConnection::run_threaded),
    );

    app.add_systems(Update, on_connect);
}

fn on_connect(mut messages: ReadStdbConnectedMessage) {
    for message in messages.read() {
        println!("SpacetimeDB module connected: {:?}", message.identity);
    }
}

// .add_table(RemoteTables::player)
// .add_table(RemoteTables::character)
// .add_table(RemoteTables::character_instance)
// .add_table(RemoteTables::transform)
// .add_reducer::<CreateCharacter>()
// .add_reducer::<EnterWorld>()
// .add_reducer::<SetMoveTarget>(),

#[derive(Debug, RegisterReducerMessage)]
#[allow(dead_code)]
pub struct Add {
    event: ReducerEvent<Reducer>,
    name: String,
}

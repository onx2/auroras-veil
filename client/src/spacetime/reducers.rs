use crate::stdb::{
    DbConnection, Reducer, RemoteModule, RemoteReducers,
    create_character_reducer::create_character, delete_character_reducer::delete_character,
    enter_world_reducer::enter_world, leave_world_reducer::leave_world,
};
use bevy_spacetimedb::RegisterReducerMessage;
use spacetimedb_sdk::ReducerEvent;

#[derive(Debug, RegisterReducerMessage)]
#[allow(dead_code)]
pub struct CreateCharacter {
    pub event: ReducerEvent<Reducer>,
    pub name: String,
}

#[derive(Debug, RegisterReducerMessage)]
pub struct DeleteCharacter {
    pub event: ReducerEvent<Reducer>,
    pub character_id: u32,
}

#[derive(Debug, RegisterReducerMessage)]
pub struct EnterWorld {
    pub event: ReducerEvent<Reducer>,
    pub character_id: u32,
}

#[derive(Debug, RegisterReducerMessage)]
pub struct LeaveWorld {
    pub event: ReducerEvent<Reducer>,
}

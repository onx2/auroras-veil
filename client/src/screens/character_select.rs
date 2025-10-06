//! The screen state for selecting a character for gameplay.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, StdbSubscriptions, SubKey},
    stdb::enter_world,
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CharacterSelect), setup);
}

fn setup(mut next_screen: ResMut<NextState<Screen>>) {
    println!("Screen::CharacterSelect -> setup")
    // todo: if this player doesn't have a character for some reason,
    // we should auto-navigate to the CreateCharacter screen
    // next_screen.set(Screen::CreateCharacter);
}

/// Given a selected character, call the spacetimeDB reducer that requests that the server
/// consider this play in the world and updates the database as needed.
fn enter_world(
    mut next_screen: ResMut<NextState<Screen>>,
    stdb: SpacetimeDB,
    mut stdb_subscriptions: ResMut<StdbSubscriptions>,
) {
    let character_id = 42;
    stdb_subscriptions.upsert(
        SubKey::CharInstanceData,
        stdb.subscription_builder()
            .subscribe("SELECT * from character_instance where identity = :sender"),
    );

    match stdb.reducers().enter_world(character_id) {
        Ok(_) => {
            next_screen.set(Screen::Gameplay);
        }
        Err(_msg) => {
            stdb_subscriptions.remove(SubKey::CharInstanceData);
        }
    }
}

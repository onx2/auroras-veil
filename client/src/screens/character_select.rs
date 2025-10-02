//! The screen state for selecting a character for gameplay.

use bevy::prelude::*;

use crate::screens::Screen;

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
fn enter_world(mut next_screen: ResMut<NextState<Screen>>) {
    // let subscribe_to_ci = stdb.subscribe("SELECT * from character_instance where identity = :sender");
    // match stdb::reducers().enter_world(character_id) {
    //     Ok(_) => {
    //         next_screen.set(Screen::Gameplay);
    //     },
    //     Err(msg) => {
    //         display_error_msg(msg);
    //         subscribe_to_ci.unsubscribe(); ????
    //     }
    // }
}

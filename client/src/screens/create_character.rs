//! The screen state for creating a new character.

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
}

fn setup() {
    println!("Screen::CreateCharacter -> setup")
}

fn create_character(mut next_screen: ResMut<NextState<Screen>>) {
    println!("Screen::CharacterSelect -> create_character")
    // match stdb.reducers().create_character() {
    //     Ok(_) => {
    //         next_screen.set(Screen::CharacterSelect);
    //     },
    //     Err(msg) => {
    //         display_error_msg(msg);
    //     }
    // }
}

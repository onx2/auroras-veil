//! The game's main screen states and transitions between them.

mod character_select;
mod create_character;
mod gameplay;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    // app.enable_state_scoped_entities::<Screen>();
    app.add_plugins((
        splash::plugin,
        title::plugin,
        gameplay::plugin,
        character_select::plugin,
        create_character::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Gameplay,
    CharacterSelect,
    CreateCharacter,
}

//! The screen state for the main gameplay.

use crate::{screens::Screen, spacetime::StdbSubscriptions};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(mut stdb_subscriptions: ResMut<StdbSubscriptions>) {
    println!("Screen::Gameplay -> setup");
    // We don't need all the character data anymore, we can specifically subscribe to the in-game character
    stdb_subscriptions.remove("title_data");
}

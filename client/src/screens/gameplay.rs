//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup() {
    println!("Screen::Gameplay -> setup")
}

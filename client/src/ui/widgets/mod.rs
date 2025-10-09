use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

pub mod button;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, TextInputPlugin));
}

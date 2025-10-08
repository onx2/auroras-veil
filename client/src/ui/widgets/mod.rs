use bevy::prelude::*;

pub mod button;
pub mod input;
pub mod window;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, input::plugin));
}

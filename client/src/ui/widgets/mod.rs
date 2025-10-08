use bevy::prelude::*;

pub mod button;
pub mod window;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin));
}

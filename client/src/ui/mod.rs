use bevy::prelude::*;

pub mod camera;
pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, widgets::plugin));
}

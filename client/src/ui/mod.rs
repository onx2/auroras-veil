use bevy::prelude::*;

pub mod camera;
pub mod theme;
pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);
    app.add_plugins(theme::plugin);
    app.add_plugins(widgets::plugin);
}

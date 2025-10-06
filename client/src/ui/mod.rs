use bevy::prelude::*;

pub mod camera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);
}

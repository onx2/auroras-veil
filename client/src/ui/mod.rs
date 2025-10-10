use bevy::prelude::*;

pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((widgets::plugin));
}

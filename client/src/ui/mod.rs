use bevy::{input_focus::InputFocus, prelude::*};

pub mod camera;
pub mod theme;
// pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, theme::plugin));
    app.init_resource::<InputFocus>();
}

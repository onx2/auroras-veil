use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_ui_camera);
}

fn add_ui_camera(mut commands: Commands) {
    commands.spawn((IsDefaultUiCamera, Camera2d));
}

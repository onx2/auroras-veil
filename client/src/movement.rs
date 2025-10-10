use bevy::prelude::*;

use crate::AppSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, systems.in_set(AppSystems::ServerUpdate));
}

fn systems() {}

use bevy::prelude::*;

use crate::AppSystems;

#[derive(Component)]
pub struct Movement {
    // pub speed: f16,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, systems.in_set(AppSystems::ServerUpdate));
}

fn systems() {}

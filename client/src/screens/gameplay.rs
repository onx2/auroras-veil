//! The screen state for the main gameplay.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, StdbSubscriptions, SubKey},
    stdb::{
        CharacterInstanceTableAccess, CharacterTableAccess, EntityTableAccess, TransformTableAccess,
    },
};
use bevy::prelude::*;
use spacetimedb_sdk::Identity;

/// A Bevy Resource holding the definitive identity and lookup context
/// for the character controlled by the local client.
#[derive(Resource, Debug)]
pub struct PlayerState {
    pub player_id: Identity,
    pub character_id: u32,
    pub race_id: u32,
    pub class_id: u32,
    pub character_instance_id: u32,
    pub entity_id: u32,
    pub transform_id: u32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    // app.add_systems(Update, update.run_if(in_state(Screen::Gameplay)));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stdb: SpacetimeDB,
    mut stdb_subscriptions: ResMut<StdbSubscriptions>,
) {
    println!("Screen::Gameplay -> setup");
    // Now that we're in game, we should get all characters
    stdb_subscriptions.remove(SubKey::OwnedCharacterData);

    let Some(ci) = stdb
        .db()
        .character_instance()
        .identity()
        .find(&stdb.identity())
    else {
        panic!("No character instance found");
    };

    let Some(c) = stdb.db().character().id().find(&ci.character_id) else {
        panic!("No character found");
    };

    let Some(e) = stdb.db().entity().id().find(&ci.entity_id) else {
        panic!("No entity found");
    };

    let Some(t) = stdb.db().transform().id().find(&e.transform_id) else {
        panic!("No transform found");
    };

    commands.insert_resource(PlayerState {
        player_id: stdb.identity(),
        character_id: ci.character_id,
        race_id: c.race_id,
        class_id: c.class_id,
        character_instance_id: ci.id,
        entity_id: ci.entity_id,
        transform_id: t.id,
    });

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

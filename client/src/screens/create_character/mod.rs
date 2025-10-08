#![allow(clippy::type_complexity)]

mod create_button;

use crate::{screens::Screen, spacetime::reducers::CreateCharacter};
use bevy::prelude::*;
use bevy_spacetimedb::ReadReducerMessage;

#[derive(Resource)]
pub struct CreateCharacterForm {
    race: u32,
    class: u32,
    name: String,
}

#[derive(Component)]
struct CreateCharacterEntity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
    app.add_systems(
        Update,
        on_character_created.run_if(in_state(Screen::CreateCharacter)),
    );
    app.add_systems(OnExit(Screen::CreateCharacter), cleanup);
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CreateCharacterEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<CreateCharacterForm>();
}

fn on_character_created(
    mut events: ReadReducerMessage<CreateCharacter>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for event in events.read() {
        println!("CreateCharacter: {:?}", event.result.event.status);
        match event.result.event.status {
            spacetimedb_sdk::Status::Committed => {
                next_screen.set(Screen::CharacterSelect);
            }
            spacetimedb_sdk::Status::Failed(ref msg) => {
                println!("Failed to create character -> Reason: {:?}", msg);
            }
            spacetimedb_sdk::Status::OutOfEnergy => {
                println!("Failed to create character -> Reason: OutOfEnergy");
            }
        }
    }
}

fn setup(mut commands: Commands) {
    println!("Screen::CreateCharacter -> setup");
    commands.insert_resource(CreateCharacterForm {
        race: 1,
        class: 1,
        name: String::new(),
    });
    commands.spawn((
        CreateCharacterEntity,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            max_width: px(800.),
            border: UiRect::all(px(1)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: px(10),
            ..default()
        },
        children![,],
    ));
}

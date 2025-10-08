//! The screen state for creating a new character.

use crate::stdb::create_character_reducer::create_character;
use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
};
use bevy::feathers::controls::ButtonProps;
use bevy::feathers::theme::ThemedText;
use bevy::ui_widgets::{Activate, observe};
use bevy::{feathers::controls::button, prelude::*};
use bevy_spacetimedb::ReadReducerMessage;

#[derive(Component)]
struct CreateCharacterEntity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
    app.add_systems(
        Update,
        on_character_create.run_if(in_state(Screen::CreateCharacter)),
    );
    app.add_systems(OnExit(Screen::CreateCharacter), cleanup);
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CreateCharacterEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
}

fn on_character_create(
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

fn setup(mut commands: Commands, stdb: SpacetimeDB) {
    println!("Screen::CreateCharacter -> setup");
    commands.spawn((
        CreateCharacterEntity,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            button(
                ButtonProps::default(),
                (),
                Spawn((Text::new("Normal"), ThemedText))
            ),
            observe(|activate: On<Activate>, stdb: SpacetimeDB| {
                println!("Screen::CreateCharacter -> create_character");
                if let Err(_) = stdb.reducers().create_character("Jeff".into(), 1, 1) {
                    println!("Unable to create character due to a networking issue.");
                }
            })
        )],
    ));
}

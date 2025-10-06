//! The screen state for creating a new character.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
    stdb::create_character,
};
use bevy::prelude::*;
use bevy_spacetimedb::ReadReducerMessage;

#[derive(Component)]
struct CreateCharacterEntity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
    app.add_systems(
        Update,
        (interact_play_button, on_character_create).run_if(in_state(Screen::CreateCharacter)),
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

fn setup(mut commands: Commands) {
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
            Button,
            Node {
                width: px(240.0),
                height: px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(px(6.0)),
            BackgroundColor(Color::srgba(0.2067, 0.1526, 0.3756, 1.)),
            children![(
                Text("Create".into()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                }
            )],
        )],
    ));
}

fn interact_play_button(
    mut buttons: Query<
        (&Interaction, Option<&mut BackgroundColor>),
        (Changed<Interaction>, With<Button>),
    >,
    stdb: SpacetimeDB,
) {
    for (interaction, bg) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                println!("Screen::CreateCharacter -> create_character");
                if let Err(_) = stdb.reducers().create_character("Jeff".into(), 1, 1) {
                    println!("Unable to create character due to a networking issue.");
                }
            }
            Interaction::Hovered => {
                if let Some(mut color) = bg {
                    *color = BackgroundColor(Color::srgba(0.2639, 0.2002, 0.455, 1.));
                }
            }
            Interaction::None => {
                if let Some(mut color) = bg {
                    *color = BackgroundColor(Color::srgba(0.2067, 0.1526, 0.3756, 1.));
                }
            }
        }
    }
}

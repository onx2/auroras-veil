#![allow(clippy::type_complexity)]

mod create_button;

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
    stdb::{ClassTableAccess, RaceTableAccess},
    ui::widgets::button::{ButtonProps, button},
};
use bevy::{prelude::*, ui_widgets::observe};
use bevy_spacetimedb::ReadReducerMessage;
use spacetimedb_sdk::Table;

#[derive(Resource)]
pub struct CreateCharacterForm {
    pub race: u32,
    pub class: u32,
    pub name: String,
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

fn setup(mut commands: Commands, stdb: SpacetimeDB) {
    println!("Screen::CreateCharacter -> setup");
    commands.insert_resource(CreateCharacterForm {
        race: 1,
        class: 1,
        name: String::new(),
    });

    // Build UI imperatively so we can add a dynamic number of race buttons
    let root = commands
        .spawn((
            CreateCharacterEntity,
            Node {
                width: percent(100),
                height: percent(100),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    let grid = commands
        .spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::auto(3),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                border: UiRect::all(px(1)),
                max_width: px(800),
                column_gap: px(20),
                ..default()
            },
            ChildOf(root),
        ))
        .id();

    // Columns
    let race_col = commands
        .spawn((
            Node {
                height: percent(100),
                border: UiRect::all(px(1)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ChildOf(grid),
        ))
        .id();
    for race in stdb.db().race().iter().collect::<Vec<_>>().iter() {
        let race_id: u32 = race.id as u32;
        commands.spawn((
            button(Spawn(Text::new(race.name.clone())), ButtonProps::default()),
            observe(
                move |_: On<Pointer<Click>>, mut form: ResMut<CreateCharacterForm>| {
                    form.race = race_id;
                },
            ),
            ChildOf(race_col),
        ));
    }

    let attribute_col = commands
        .spawn((
            Node {
                height: percent(100),
                border: UiRect::all(px(1)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ChildOf(grid),
        ))
        .id();

    // TODO button placeholder under attribute column
    commands.spawn((
        button(Spawn(Text::new("TODO")), ButtonProps::default()),
        ChildOf(attribute_col),
    ));

    let class_col = commands
        .spawn((
            Node {
                height: percent(100),
                border: UiRect::all(px(1)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ChildOf(grid),
        ))
        .id();

    for class in stdb.db().class().iter().collect::<Vec<_>>().iter() {
        let class_id: u32 = class.id as u32;
        commands.spawn((
            button(Spawn(Text::new(class.name.clone())), ButtonProps::default()),
            observe(
                move |_: On<Pointer<Click>>, mut form: ResMut<CreateCharacterForm>| {
                    form.class = class_id;
                },
            ),
            ChildOf(class_col),
        ));
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CreateCharacterEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<CreateCharacterForm>();
}

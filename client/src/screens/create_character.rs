#![allow(clippy::type_complexity)]
//! Create Character screen: two-row layout with form wiring.
//!
//! Layout
//! - Row 1: three columns with headings: Race, Attributes, Class
//! - Row 2: left text input (character name), right Create button
//!
//! Wiring
//! - TextInputValue is mirrored into a resource `CreateCharacterForm`
//! - Clicking Create submits `form.name` via the reducer

use crate::stdb::create_character_reducer::create_character;
use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
    ui::widgets::{
        button::{ButtonProps, button},
        input::{InputProps, text_input},
    },
};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};
use bevy_simple_text_input::TextInputValue;
use bevy_spacetimedb::ReadReducerMessage;

#[derive(Component)]
struct CreateCharacterEntity;

#[derive(Component)]
struct CreateCharacterUiRoot;

#[derive(Component)]
struct CreateCharacterNameInput;

// External form state that mirrors the TextInput field value.
#[derive(Resource, Default)]
struct CreateCharacterForm {
    name: String,
}

#[derive(SystemParam)]
struct CCParams<'w> {
    stdb: SpacetimeDB<'w>,
    form: Res<'w, CreateCharacterForm>,
}

impl ImmediateAttach<CapsUi> for CreateCharacterUiRoot {
    type Params = CCParams<'static>;

    fn construct(ui: &mut Imm<CapsUi>, params: &mut CCParams) {
        let root = ui.ch().on_spawn_insert(|| {
            (Node {
                flex_direction: FlexDirection::Column,
                width: percent(100.0),
                max_width: px(680.0),
                row_gap: px(16.0),
                ..default()
            },)
        });
        root.add(|ui| {
            // Row 1: three columns with headings
            let row1 = ui.ch().on_spawn_insert(|| {
                (Node {
                    flex_direction: FlexDirection::Row,
                    width: percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
                    column_gap: px(16.0),
                    ..default()
                },)
            });
            row1.add(|ui| {
                // Column 1: Race
                let col1 = ui.ch_id("race_column").on_spawn_insert(|| {
                    (Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8.0),
                        ..default()
                    },)
                });
                col1.add(|ui| {
                    // Column heading
                    ui.ch().on_spawn_insert(|| {
                        (
                            Text::new("Race"),
                            TextFont {
                                font: Handle::<Font>::default(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )
                    });
                });

                // Column 2: Attributes
                let col2 = ui.ch().on_spawn_insert(|| {
                    (Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8.0),
                        ..default()
                    },)
                });
                col2.add(|ui| {
                    // Column heading
                    ui.ch().on_spawn_insert(|| {
                        (
                            Text::new("Attributes"),
                            TextFont {
                                font: Handle::<Font>::default(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )
                    });
                });

                // Column 3: Class
                let col3 = ui.ch().on_spawn_insert(|| {
                    (Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8.0),
                        ..default()
                    },)
                });
                col3.add(|ui| {
                    // Column heading
                    ui.ch().on_spawn_insert(|| {
                        (
                            Text::new("Class"),
                            TextFont {
                                font: Handle::<Font>::default(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )
                    });
                });
            });
        });

        // Row 2: left text input and right create button
        ui.ch()
            .on_spawn_insert(|| {
                (Node {
                    flex_direction: FlexDirection::Row,
                    width: percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },)
            })
            .add(|ui| {
                // Left: character name input
                let input_props = InputProps::default()
                    .size(Val::Px(360.0), Val::Px(48.0))
                    .placeholder("Choose a name");
                text_input(ui, "create_character_name_input", input_props);

                // Right: create button
                let btn_props = ButtonProps::default()
                    .size(Val::Px(240.0), Val::Px(72.0))
                    .padding(UiRect::axes(Val::Px(20.0), Val::Px(12.0)));

                if button(ui, "create_character_btn", "Create", btn_props).clicked {
                    let name = params.form.name.clone();
                    println!(
                        "Screen::CreateCharacter -> create_character(name = {:?})",
                        name
                    );
                    if let Err(_) = params.stdb.reducers().create_character(name, 1, 1) {
                        println!("Unable to create character due to a networking issue.");
                    }
                }
            });

        // Perform reducer call after building UI so we can use ECS params here
    }
}

pub(super) fn plugin(app: &mut App) {
    // Ensure resources exist
    app.insert_resource(CreateCharacterForm::default());

    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, CreateCharacterUiRoot>::new());

    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
    app.add_systems(
        Update,
        (on_character_create, sync_name_input_on_change).run_if(in_state(Screen::CreateCharacter)),
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

fn sync_name_input_on_change(
    mut form: ResMut<CreateCharacterForm>,
    q: Query<&TextInputValue, (Changed<TextInputValue>)>,
) {
    if let Some(v) = q.iter().next() {
        form.name = v.0.clone();
    }
}

fn setup(mut commands: Commands) {
    println!("Screen::CreateCharacter -> setup");
    commands.spawn((
        CreateCharacterEntity,
        CreateCharacterUiRoot,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));
}

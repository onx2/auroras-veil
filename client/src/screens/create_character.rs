use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
    stdb::{ClassTableAccess, RaceTableAccess, create_character},
    ui::widgets::button::{ButtonProps, ButtonVariant, button},
};
use bevy::{prelude::*, ui_widgets::observe};
use bevy_simple_text_input::{TextInput, TextInputTextColor, TextInputTextFont, TextInputValue};
use bevy_spacetimedb::ReadReducerMessage;
use spacetimedb_sdk::Table;

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Resource)]
pub struct CreateCharacterState {
    pub race: u32,
    pub class: u32,
    pub name: String,
}

fn bind_name_to_state(
    mut state: ResMut<CreateCharacterState>,
    query: Query<&TextInputValue, With<TextInput>>,
) {
    if let Ok(value) = query.single() {
        let input_value = &value.0;
        if state.name != *input_value {
            state.name = input_value.clone();
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
    app.add_systems(
        Update,
        (on_character_created, bind_name_to_state).run_if(in_state(Screen::CreateCharacter)),
    );
    app.add_systems(OnExit(Screen::CreateCharacter), |mut commands: Commands| {
        commands.remove_resource::<CreateCharacterState>();
    });
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
    commands.insert_resource(CreateCharacterState {
        race: 1,
        class: 1,
        name: String::from(""),
    });

    // Build UI imperatively so we can add a dynamic number of race buttons
    let root = commands
        .spawn((
            DespawnOnExit(Screen::CreateCharacter),
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

    // Bottom row
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(200.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor::all(BORDER_COLOR_ACTIVE),
                BackgroundColor(BACKGROUND_COLOR),
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 34.,
                    ..default()
                }),
                TextInputTextColor(TextColor(TEXT_COLOR)),
            ),
            (
                button(
                    Spawn(Text::new("Create")),
                    ButtonProps {
                        variant: ButtonVariant::Primary,
                        ..default()
                    },
                ),
                observe(
                    |_: On<Pointer<Click>>, stdb: SpacetimeDB, state: Res<CreateCharacterState>| {
                        if state.name.is_empty() {
                            println!("Name cannot be empty.");
                            return;
                        }
                        if let Err(_) = stdb.reducers().create_character(
                            state.name.clone(),
                            state.race,
                            state.class,
                        ) {
                            println!("Unable to create character due to a networking issue.");
                        }
                    },
                ),
            )
        ],
        ChildOf(root),
    ));
    // Columns
    let race_col = commands
        .spawn((
            Node {
                height: percent(100),
                border: UiRect::all(px(1)),
                display: Display::Flex,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            ChildOf(grid),
        ))
        .id();
    for race in stdb.db().race().iter().collect::<Vec<_>>().iter() {
        let race_id = race.id;
        commands.spawn((
            button(Spawn(Text::new(race.name.clone())), ButtonProps::default()),
            observe(
                move |_: On<Pointer<Click>>, mut state: ResMut<CreateCharacterState>| {
                    state.race = race_id;
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
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor::all(Color::WHITE),
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
            BorderColor::all(Color::WHITE),
            ChildOf(grid),
        ))
        .id();

    for class in stdb.db().class().iter().collect::<Vec<_>>().iter() {
        let class_id = class.id;
        commands.spawn((
            button(Spawn(Text::new(class.name.clone())), ButtonProps::default()),
            observe(
                move |_: On<Pointer<Click>>, mut state: ResMut<CreateCharacterState>| {
                    state.class = class_id;
                },
            ),
            ChildOf(class_col),
        ));
    }
}

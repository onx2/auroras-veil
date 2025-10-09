//! The screen state for selecting a character for gameplay.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, StdbSubscriptions, SubKey, reducers::EnterWorld},
    stdb::{CharacterTableAccess, enter_world},
    ui::widgets::button::{ButtonProps, button},
};
use bevy::{prelude::*, ui_widgets::observe};
use bevy_spacetimedb::ReadReducerMessage;
use spacetimedb_sdk::Table;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::CharacterSelect), setup);
    app.add_systems(
        Update,
        (on_enter_world, update_character_name_text).run_if(in_state(Screen::CharacterSelect)),
    );
    app.add_systems(OnExit(Screen::CharacterSelect), |mut commands: Commands| {
        commands.remove_resource::<CharacterSelectState>();
    });
}

#[derive(Resource)]
struct CharacterSelectState {
    selected_character: Option<u32>,
}

#[derive(Component)]
struct CharacterNameText;

fn update_character_name_text(
    mut query: Query<&mut Text, With<CharacterNameText>>,
    state: Res<CharacterSelectState>,
    stdb: SpacetimeDB,
) {
    if let Some(id) = state.selected_character {
        let Ok(mut text) = query.single_mut() else {
            return;
        };
        let Some(char) = stdb.db().character().id().find(&id) else {
            return;
        };
        text.0 = char.name.clone();
    }
}

fn setup(mut commands: Commands, stdb: SpacetimeDB) {
    println!("Screen::CharacterSelect -> setup");
    commands.insert_resource(CharacterSelectState {
        selected_character: None,
    });

    let root = commands
        .spawn((
            DespawnOnExit(Screen::CharacterSelect),
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

    commands.spawn((Text::new(""), CharacterNameText, ChildOf(root)));

    let race_col = commands
        .spawn((
            Node {
                height: px(500),
                border: UiRect::all(px(1)),
                display: Display::Flex,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            ChildOf(root),
        ))
        .id();
    for character in stdb.db().character().iter().collect::<Vec<_>>().iter() {
        let character_id = character.id;
        commands.spawn((
            button(
                Spawn(Text::new(character.name.clone())),
                ButtonProps::default(),
            ),
            observe(
                move |_: On<Pointer<Click>>, mut state: ResMut<CharacterSelectState>| {
                    state.selected_character = Some(character_id);
                },
            ),
            ChildOf(race_col),
        ));
    }
    commands.spawn((
        (
            button(Spawn(Text::new("Enter World")), ButtonProps::default()),
            observe(
                |_: On<Pointer<Click>>,
                 stdb: SpacetimeDB,
                 mut stdb_subscriptions: ResMut<StdbSubscriptions>,
                 state: Res<CharacterSelectState>| {
                    if let Some(character_id) = state.selected_character {
                        stdb_subscriptions.upsert(
                            SubKey::CharInstanceData,
                            stdb.subscription_builder().subscribe(
                                "SELECT * from character_instance where identity = :sender",
                            ),
                        );

                        if let Err(_) = stdb.reducers().enter_world(character_id) {
                            stdb_subscriptions.remove(SubKey::CharInstanceData);
                        }
                    }
                },
            ),
        ),
        ChildOf(root),
    ));
    commands.spawn((
        button(Spawn(Text::new("Create New")), ButtonProps::default()),
        observe(
            |_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>| {
                next_screen.set(Screen::CreateCharacter);
            },
        ),
        ChildOf(root),
    ));
}

fn on_enter_world(
    mut events: ReadReducerMessage<EnterWorld>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut stdb_subscriptions: ResMut<StdbSubscriptions>,
) {
    for event in events.read() {
        println!("EnterWorld event: {:?}", event.result.event.status);
        match event.result.event.status {
            spacetimedb_sdk::Status::Committed => {
                println!("Entering world...");
                next_screen.set(Screen::Gameplay);
            }
            spacetimedb_sdk::Status::Failed(ref msg) => {
                println!("Failed to enter world -> Reason: {:?}", msg);
                stdb_subscriptions.remove(SubKey::CharInstanceData);
            }
            spacetimedb_sdk::Status::OutOfEnergy => {
                println!("Failed to enter world -> Reason: OutOfEnergy");
                stdb_subscriptions.remove(SubKey::CharInstanceData);
            }
        }
    }
}

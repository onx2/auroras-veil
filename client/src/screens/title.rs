//! The title screen that appears after the splash screen.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, StdbSubscriptions, SubKey},
    stdb::CharacterTableAccess,
};
use bevy::{
    prelude::*,
    ui_widgets::observe,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResolution},
};
use spacetimedb_sdk::Table;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup);
    app.add_systems(Update, subscribe_to_data.run_if(in_state(Screen::Title)));
}

fn subscribe_to_data(stdb: SpacetimeDB, mut stdb_subscriptions: ResMut<StdbSubscriptions>) {
    if stdb.is_active() && !stdb_subscriptions.contains(SubKey::OwnedCharacterData) {
        stdb_subscriptions.upsert(
            SubKey::OwnedCharacterData,
            stdb.subscription_builder()
                .subscribe("SELECT * FROM character WHERE identity = :sender"),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
    monitor_q: Query<&Monitor, With<PrimaryMonitor>>,
) {
    println!("Title -> setup");
    let Ok(monitor) = monitor_q.single() else {
        panic!("No monitor, how were you expecting to play the game?");
    };

    let Ok(mut window) = window_q.single_mut() else {
        panic!("No window, how were you expecting to play the game?");
    };

    commands.insert_resource(ClearColor(Color::BLACK));
    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    window.position = WindowPosition::At(IVec2::new(0, 0));
    window.decorations = true;
    window.resizable = false;
    window.resolution = WindowResolution::default();

    commands.spawn((
        DespawnOnExit(Screen::Title),
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
                width: px(150),
                height: px(65),
                border: UiRect::all(px(5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BorderRadius::MAX,
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Play"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )],
            observe(
                |_: On<Pointer<Click>>,
                 stdb: SpacetimeDB,
                 mut next_screen: ResMut<NextState<Screen>>| {
                    if stdb.db().character().iter().next().is_some() {
                        next_screen.set(Screen::CharacterSelect);
                    } else {
                        next_screen.set(Screen::CreateCharacter);
                    }
                }
            )
        )],
    ));
}

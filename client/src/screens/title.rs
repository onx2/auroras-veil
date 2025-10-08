//! The title screen that appears after the splash screen.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, StdbSubscriptions, SubKey},
    stdb::CharacterTableAccess,
    ui::theme::tokens,
};
use bevy::{
    feathers::{
        controls::{ButtonProps, button},
        theme::ThemedText,
    },
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, observe},
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResolution},
};
use spacetimedb_sdk::Table;

#[derive(Component)]
struct TitleEntity;

pub(super) fn plugin(app: &mut App) {
    // app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, TitleUiRoot>::new());
    app.add_systems(OnEnter(Screen::Title), setup);
    app.add_systems(Update, subscribe_to_data.run_if(in_state(Screen::Title)));
    app.add_systems(OnExit(Screen::Title), cleanup);
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
    // window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    window.mode = WindowMode::Windowed;
    window.resizable = false;
    window.decorations = false;
    window.position = WindowPosition::At(IVec2::new(0, 0));
    window.resolution = WindowResolution::new(monitor.physical_width, monitor.physical_height);

    commands.spawn((
        TitleEntity,
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            (
                Node {
                    height: px(40.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(px(8.0), px(0.)),
                    // flex_grow: 1.0,
                    ..Default::default()
                },
                Button,
                BackgroundColor(Color::srgb(58., 45., 33.)),
                Hovered::default(),
                TabIndex(0),
                children![(
                    Text::new("Play"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ),
            observe(on_play_press),
        )],
    ));
}

fn on_play_press(_: On<Activate>, mut next_screen: ResMut<NextState<Screen>>, stdb: SpacetimeDB) {
    println!("Play!");
    if stdb.db().character().iter().next().is_some() {
        next_screen.set(Screen::CharacterSelect);
    } else {
        next_screen.set(Screen::CreateCharacter);
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<TitleEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<ClearColor>();
}

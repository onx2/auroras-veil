//! The title screen that appears after the splash screen.

use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, StdbSubscriptions, SubKey},
    stdb::CharacterTableAccess,
    ui::widgets::button::{ButtonProps, button_id},
};
use bevy::{ecs::system::SystemParam, window::WindowMode};
use bevy::{
    prelude::*,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowResolution},
};
use bevy_immediate::ui::CapsUi;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
};
use spacetimedb_sdk::Table;

#[derive(Component)]
struct TitleEntity;

#[derive(Component)]
struct TitleUiRoot;

#[derive(SystemParam)]
struct TitleParams<'w> {
    next_screen: ResMut<'w, NextState<Screen>>,
    stdb: SpacetimeDB<'w>,
}

impl ImmediateAttach<CapsUi> for TitleUiRoot {
    type Params = TitleParams<'static>;

    fn construct(ui: &mut Imm<CapsUi>, params: &mut TitleParams) {
        let props = ButtonProps::default()
            .size(Val::Px(240.0), Val::Px(80.0))
            .padding(UiRect::axes(Val::Px(20.0), Val::Px(12.0)));
        let props2 = props.clone().disabled(true);
        let res = button_id(ui, "play_btn", "Play", props);
        let res2 = button_id(ui, "play_btn2", "Play", props2);
        if res2.clicked {
            println!("shouldnt fire")
        }
        if res.clicked {
            if params.stdb.db().character().iter().next().is_some() {
                params.next_screen.set(Screen::CharacterSelect);
            } else {
                params.next_screen.set(Screen::CreateCharacter);
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, TitleUiRoot>::new());
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
    window.position = WindowPosition::At(IVec2::new(0, 0));
    window.resolution = WindowResolution::new(monitor.physical_width, monitor.physical_height);

    commands.spawn((
        TitleEntity,
        TitleUiRoot,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<TitleEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<ClearColor>();
}

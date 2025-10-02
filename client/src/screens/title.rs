//! The title screen that appears after the splash screen.

use bevy::{
    prelude::*,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowResolution},
};

use crate::screens::Screen;

#[derive(Component)]
struct TitleEntity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup);
    app.add_systems(Update, interact_play_button.run_if(in_state(Screen::Title)));
    app.add_systems(OnExit(Screen::Title), cleanup);
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
    window.position = WindowPosition::At(IVec2::new(0, 0));
    window.resolution = WindowResolution::new(monitor.physical_width, monitor.physical_height);

    // Spawn a UI camera for the title screen.
    commands.spawn((TitleEntity, Camera2d));

    // Fullscreen root node to center the Play button.
    commands.spawn((
        TitleEntity,
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
                Text("Play".into()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                }
            )]
        )],
    ));
}

fn interact_play_button(
    mut next_screen: ResMut<NextState<Screen>>,
    mut buttons: Query<
        (&Interaction, Option<&mut BackgroundColor>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, bg) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                // todo: dynamically switch to the right screen when pressing play based on if this player already
                // has a character created to simplify the onboarding flow.
                // if has_character {
                //     next_screen.set(Screen::CharacterSelect);
                // } else {
                //     next_screen.set(Screen::CreateCharacter);
                // }
                next_screen.set(Screen::CreateCharacter);
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

fn cleanup(mut commands: Commands, entities: Query<Entity, With<TitleEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<ClearColor>();
}

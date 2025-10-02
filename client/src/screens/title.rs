//! The title screen that appears after the splash screen.

use bevy::{
    prelude::*,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResolution},
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup);
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
}

fn on_play_button_press(mut next_screen: ResMut<NextState<Screen>>) {
    // todo: dynamically switch to the right screen when pressing play based on if this player already
    // has a character created to simplify the onboarding flow.
    // if has_character {
    //     next_screen.set(Screen::CharacterSelect);
    // } else {
    //     next_screen.set(Screen::CreateCharacter);
    // }
}

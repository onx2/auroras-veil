//! The screen state when first launching the game.

use bevy::{
    prelude::*,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResolution},
};

use crate::screens::Screen;

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Component)]
struct SplashEntity;

const SPLASH_SIZE: u32 = 1024;
const FADE_DURATION: f32 = 1.0;
const WAIT_DURATION: f32 = 3.0;
const TOTAL_DURATION: f32 = FADE_DURATION + WAIT_DURATION + FADE_DURATION;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), setup);
    app.add_systems(Update, tick.run_if(in_state(Screen::Splash)));
}

fn setup(
    mut commands: Commands,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
    monitor_q: Query<&Monitor, With<PrimaryMonitor>>,
    asset_server: Res<AssetServer>,
) {
    println!("Screen::Splash -> setup");
    let Ok(monitor) = monitor_q.single() else {
        panic!("No monitor, how were you expecting to play the game?");
    };

    let Ok(mut window) = window_q.single_mut() else {
        panic!("No window, how were you expecting to play the game?");
    };

    let splash_image = asset_server.load("embedded/splash_screen.png");
    commands.insert_resource(ClearColor(Color::NONE));
    window.mode = WindowMode::Windowed;
    window.resolution = WindowResolution::new(SPLASH_SIZE, SPLASH_SIZE);
    window.position = WindowPosition::At(IVec2::new(
        ((monitor.physical_width - SPLASH_SIZE) / 2) as i32,
        ((monitor.physical_height - SPLASH_SIZE) / 2) as i32,
    ));

    commands.spawn((
        SplashEntity,
        ImageNode {
            image: splash_image,
            color: Color::srgba(0., 0., 0., 0.),
            ..default()
        },
        // Camera2d,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
        BorderRadius::all(px(8.0)),
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(
        TOTAL_DURATION,
        TimerMode::Once,
    )));
}

fn tick(
    time: Res<Time>,
    entities: Query<Entity, With<SplashEntity>>,
    mut images: Query<&mut ImageNode, With<SplashEntity>>,
    mut commands: Commands,
    mut splash_timer: ResMut<SplashTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if splash_timer.0.tick(time.delta()).just_finished() {
        for e in &entities {
            commands.entity(e).despawn();
        }
        commands.remove_resource::<ClearColor>();
        commands.remove_resource::<SplashTimer>();
        next_screen.set(Screen::Title);
    } else {
        let elapsed = splash_timer.0.elapsed_secs();
        let alpha = if elapsed < FADE_DURATION {
            elapsed / FADE_DURATION
        } else if elapsed < FADE_DURATION + WAIT_DURATION {
            1.0
        } else {
            let out_t = elapsed - (FADE_DURATION + WAIT_DURATION);
            1.0 - (out_t / FADE_DURATION)
        };

        for mut image in &mut images {
            image.color = Color::srgba(alpha, alpha, alpha, alpha);
        }
    }
}

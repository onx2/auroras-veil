// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod dev_tools;

mod cursor;
mod screens;
mod spacetime;
mod stdb;
mod theme;
mod ui;

#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use bevy::{asset::embedded_asset, prelude::*};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(AppPlugin);
    embedded_asset!(app, "../assets/embedded/splash_screen.png");

    app.run()
}

struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "Aurora's Veil".to_string(),
                    fit_canvas_to_parent: true,
                    // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                    transparent: true,
                    // Disabling window decorations to make it feel more like a widget than a window
                    decorations: false,
                    #[cfg(target_os = "macos")]
                    composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                    #[cfg(target_os = "linux")]
                    composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                    ..default()
                }
                .into(),
                ..default()
            }),
        );
        app.add_plugins((
            spacetime::plugin,
            screens::plugin,
            ui::plugin,
            cursor::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ));

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

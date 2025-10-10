//! The screen state for the main gameplay.

use crate::{
    screens::Screen,
    ui::widgets::button::{ButtonIcon, IconButtonProps, icon_button},
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(mut commands: Commands) {
    println!("Screen::Gameplay -> setup");

    commands.spawn((
        Node {
            height: px(500),
            width: px(500),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(px(150)),
            column_gap: px(10),
            ..default()
        },
        children![
            icon_button(IconButtonProps {
                icon: ButtonIcon::Spells,
                ..default()
            },),
            icon_button(IconButtonProps {
                icon: ButtonIcon::Character,
                ..default()
            },),
            icon_button(IconButtonProps {
                icon: ButtonIcon::Settings,
                ..default()
            },),
            icon_button(IconButtonProps {
                icon: ButtonIcon::Journal,
                ..default()
            },),
        ],
    ));
}

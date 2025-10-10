use bevy::prelude::*;

pub mod button;
pub mod icon_button;

pub use button::*;
pub use icon_button::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, icon_button::plugin));
}

#[derive(Default, Clone, Reflect, Debug, PartialEq, Eq)]
#[reflect(Clone, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
}

#[derive(Default, Clone, Reflect, Debug, PartialEq, Eq)]
#[reflect(Clone, Default)]
pub enum ButtonIcon {
    #[default]
    Spells,
    Journal,
    Settings,
    Character,
}

use crate::ui::theme::{ThemeBackgroundColor, ThemeFontColor, tokens};
use bevy::hierarchy::BuildChildren;

use bevy::{
    prelude::*,
    ui::{Interaction, InteractionDisabled},
};

/// Color variants for buttons. This also functions as a component used by the dynamic styling
/// system to identify which entities are buttons.
#[derive(Component, Default, Clone, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component, Clone, Default)]
pub enum ButtonVariant {
    /// The standard button appearance
    #[default]
    Normal,
    /// A button with a more prominent color, this is used for "call to action" buttons,
    /// default buttons for dialog boxes, and so on.
    Primary,
}

/// Parameters for the button template, passed to [`button`] function.
#[derive(Default)]
pub struct ButtonProps {
    /// Color variant for the button.
    pub variant: ButtonVariant,
}

pub fn button(
    parent: &mut bevy::hierarchy::ChildBuilder,
    size: (Val, Val),
    props: ButtonProps,
    children: impl FnOnce(&mut bevy::hierarchy::ChildBuilder),
) -> Entity {
    let (bg_token, text_token) = match props.variant {
        ButtonVariant::Normal => (tokens::BUTTON_BG, tokens::BUTTON_TEXT),
        ButtonVariant::Primary => (tokens::BUTTON_PRIMARY_BG, tokens::BUTTON_PRIMARY_TEXT),
    };

    let (width, height) = size;

    parent
        .spawn((
            bevy::ui::node_bundles::NodeBundle {
                style: bevy::ui::Style {
                    width,
                    height,
                    align_items: bevy::ui::AlignItems::Center,
                    justify_content: bevy::ui::JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            bevy::ui::widget::Button,
            props.variant,
            ThemeBackgroundColor(bg_token),
            ThemeFontColor(text_token),
        ))
        .with_children(|cb| {
            children(cb);
        })
        .id()
}

fn update_button_styles(
    q_buttons: Query<
        (
            Entity,
            &ButtonVariant,
            Has<InteractionDisabled>,
            &Interaction,
            &ThemeBackgroundColor,
            &ThemeFontColor,
        ),
        Or<(
            Changed<Interaction>,
            Changed<ButtonVariant>,
            Added<InteractionDisabled>,
        )>,
    >,
    mut commands: Commands,
) {
    for (button_ent, variant, disabled, interaction, bg_color, font_color) in q_buttons.iter() {
        set_button_styles(
            button_ent,
            variant,
            disabled,
            interaction,
            bg_color,
            font_color,
            &mut commands,
        );
    }
}

fn update_button_styles_remove(
    q_buttons: Query<(
        Entity,
        &ButtonVariant,
        Has<InteractionDisabled>,
        &Interaction,
        &ThemeBackgroundColor,
        &ThemeFontColor,
    )>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
    mut commands: Commands,
) {
    removed_disabled.read().for_each(|ent| {
        if let Ok((button_ent, variant, disabled, interaction, bg_color, font_color)) =
            q_buttons.get(ent)
        {
            set_button_styles(
                button_ent,
                variant,
                disabled,
                interaction,
                bg_color,
                font_color,
                &mut commands,
            );
        }
    });
}

fn set_button_styles(
    button_ent: Entity,
    variant: &ButtonVariant,
    disabled: bool,
    interaction: &Interaction,
    bg_color: &ThemeBackgroundColor,
    font_color: &ThemeFontColor,
    commands: &mut Commands,
) {
    let bg_token = match (variant, disabled, interaction) {
        (ButtonVariant::Normal, true, _) => tokens::BUTTON_BG_DISABLED,
        (ButtonVariant::Normal, false, Interaction::Pressed) => tokens::BUTTON_BG_PRESSED,
        (ButtonVariant::Normal, false, Interaction::Hovered) => tokens::BUTTON_BG_HOVER,
        (ButtonVariant::Normal, false, Interaction::None) => tokens::BUTTON_BG,
        (ButtonVariant::Primary, true, _) => tokens::BUTTON_PRIMARY_BG_DISABLED,
        (ButtonVariant::Primary, false, Interaction::Pressed) => tokens::BUTTON_PRIMARY_BG_PRESSED,
        (ButtonVariant::Primary, false, Interaction::Hovered) => tokens::BUTTON_PRIMARY_BG_HOVER,
        (ButtonVariant::Primary, false, Interaction::None) => tokens::BUTTON_PRIMARY_BG,
    };

    let font_color_token = match (variant, disabled) {
        (ButtonVariant::Normal, true) => tokens::BUTTON_TEXT_DISABLED,
        (ButtonVariant::Normal, false) => tokens::BUTTON_TEXT,
        (ButtonVariant::Primary, true) => tokens::BUTTON_PRIMARY_TEXT_DISABLED,
        (ButtonVariant::Primary, false) => tokens::BUTTON_PRIMARY_TEXT,
    };

    // Change background color
    if bg_color.0 != bg_token {
        commands
            .entity(button_ent)
            .insert(ThemeBackgroundColor(bg_token));
    }

    // Change font color
    if font_color.0 != font_color_token {
        commands
            .entity(button_ent)
            .insert(ThemeFontColor(font_color_token));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_button_styles, update_button_styles_remove));
}

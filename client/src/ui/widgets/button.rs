use bevy::prelude::*;
use bevy::ui::BorderRadius;
use bevy_immediate::ui::ImplCapsUi;
use bevy_immediate::ui::clicked::ImmUiClicked;
use bevy_immediate::{CapSet, Imm};

/// Plugin that drives the visual state (normal/hover/pressed/disabled)
/// of all `UiButton` components every frame.
pub struct UiButtonPlugin;

impl Plugin for UiButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_button_look);
    }
}

/// Internal marker placed on the text entity of a button so we can
/// update its color when the button state changes.
#[derive(Component)]
pub struct UiButtonLabel;

/// A reusable, styled UI button marker and its visual parameters.
#[derive(Component, Clone)]
pub struct UiButton {
    pub disabled: bool,
    pub style: ButtonStyle,
}

/// Button visuals approximating the "Create" button in the provided mock.
/// You can override any field through `ButtonProps::style(...)`.
#[derive(Clone)]
pub struct ButtonStyle {
    pub bg: Color,
    pub bg_hover: Color,
    pub bg_pressed: Color,
    pub bg_disabled: Color,

    pub border: Color,
    pub border_hover: Color,
    pub border_pressed: Color,

    pub text: Color,
    pub text_disabled: Color,

    pub border_px: f32,
    pub radius_px: f32,
    pub font_size: f32,
    pub uppercase: bool,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        // Sepia/gold palette close to the screenshot.
        Self {
            bg: Color::srgb(0.24, 0.18, 0.12), // Color::srgb(0.34, 0.26, 0.16),
            bg_hover: Color::srgb(0.40, 0.30, 0.18),
            bg_pressed: Color::srgb(0.48, 0.36, 0.22),
            bg_disabled: Color::srgba(0.24, 0.18, 0.12, 0.5),

            border: Color::srgb(0.50, 0.38, 0.26),
            border_hover: Color::srgb(0.44, 0.34, 0.24),
            border_pressed: Color::srgb(0.50, 0.38, 0.26),

            text: Color::srgb(0.88, 0.82, 0.64),
            text_disabled: Color::srgb(0.66, 0.60, 0.44),

            border_px: 2.0,
            radius_px: 2.0,
            font_size: 28.0,
            uppercase: true,
        }
    }
}

/// Builder-style input for creating a button.
#[derive(Clone)]
pub struct ButtonProps {
    pub width: Val,
    pub height: Val,
    pub padding: UiRect,
    pub style: ButtonStyle,
    pub disabled: bool,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Px(64.0),
            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
            style: ButtonStyle::default(),
            disabled: false,
        }
    }
}

impl ButtonProps {
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn padding(mut self, pad: UiRect) -> Self {
        self.padding = pad;
        self
    }
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Result when spawning a button using the immediate builder.
pub struct ButtonResponse {
    /// True on the frame the button is clicked.
    pub clicked: bool,
}

/// Spawn a styled button with a stable identifier.
///
/// Use this variant if you can provide a unique `id` among siblings; it helps
/// bevy_immediate keep the entity stable when the UI tree changes.
/// For quick prototyping, see [`button`].
///
/// Example usage inside a UI build:
/// let res = button_id(ui, ("create", 0), "CREATE", ButtonProps::default());
/// if res.clicked { /* handle click */ }
pub fn button_id<Caps, Id>(
    ui: &mut Imm<Caps>,
    id: Id,
    label: impl Into<String>,
    props: ButtonProps,
) -> ButtonResponse
where
    Caps: CapSet + ImplCapsUi,
    Id: Clone + std::hash::Hash + Eq + Send + Sync + 'static,
{
    let label_string = label.into();
    let display = if props.style.uppercase {
        label_string.to_uppercase()
    } else {
        label_string.clone()
    };

    let ButtonStyle {
        bg,
        border,
        border_px,
        radius_px,
        font_size,
        text,
        ..
    } = props.style.clone();

    // Build the button container.
    let mut b = ui
        .ch_id(id)
        .on_spawn_insert(move || {
            (
                Button,
                Node {
                    width: props.width,
                    height: props.height,
                    padding: props.padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(border_px)),
                    ..default()
                },
                BackgroundColor(bg),
                BorderColor::all(border),
                BorderRadius::all(Val::Px(radius_px)),
                UiButton {
                    disabled: props.disabled,
                    style: props.style.clone(),
                },
            )
        })
        .add(|ui| {
            ui.ch().on_spawn_insert(move || {
                (
                    Text(display),
                    TextFont {
                        font: Handle::<Font>::default(),
                        font_size,
                        ..default()
                    },
                    TextColor(text),
                    UiButtonLabel,
                )
            });
        });

    // Only report clicks if the button is not disabled.
    let clicked = if props.disabled { false } else { b.clicked() };

    ButtonResponse { clicked }
}

/// Spawn a styled button without explicitly providing an id.
/// Only use when the button's presence/order is stable frame-to-frame.
/// Prefer [`button_id`] when the UI tree can change.
pub fn button<Caps>(
    ui: &mut Imm<Caps>,
    label: impl Into<String>,
    props: ButtonProps,
) -> ButtonResponse
where
    Caps: CapSet + ImplCapsUi,
{
    // Fallback: create a child without an explicit id.
    // This is okay for static UIs, but prefer button_id(...) otherwise.
    let label_string = label.into();
    let display = if props.style.uppercase {
        label_string.to_uppercase()
    } else {
        label_string
    };

    let ButtonStyle {
        bg,
        border,
        border_px,
        radius_px,
        font_size,
        text,
        ..
    } = props.style.clone();

    let mut b = ui
        .ch()
        .on_spawn_insert(move || {
            (
                Button,
                Node {
                    width: props.width,
                    height: props.height,
                    padding: props.padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(border_px)),
                    ..default()
                },
                BackgroundColor(bg),
                BorderColor::all(border),
                BorderRadius::all(Val::Px(radius_px)),
                UiButton {
                    disabled: props.disabled,
                    style: props.style.clone(),
                },
            )
        })
        .add(|ui| {
            ui.ch().on_spawn_insert(move || {
                (
                    Text(display),
                    TextFont {
                        font: Handle::<Font>::default(),
                        font_size,
                        ..default()
                    },
                    TextColor(text),
                    UiButtonLabel,
                )
            });
        });

    let clicked = if props.disabled { false } else { b.clicked() };

    ButtonResponse { clicked }
}

/// System that applies hover/pressed/disabled visuals for all UiButtons.
fn apply_button_look(
    mut q_buttons: Query<(
        &UiButton,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut q_text: Query<&mut TextColor, With<UiButtonLabel>>,
) {
    for (button, interaction, mut bg, mut border, children) in q_buttons.iter_mut() {
        let style = &button.style;

        let (bg_color, border_color, text_color) = if button.disabled {
            (style.bg_disabled, style.border, style.text_disabled)
        } else {
            match *interaction {
                Interaction::Pressed => (style.bg_pressed, style.border_pressed, style.text),
                Interaction::Hovered => (style.bg_hover, style.border_hover, style.text),
                Interaction::None => (style.bg, style.border, style.text),
            }
        };

        if bg.0 != bg_color {
            bg.0 = bg_color;
        }
        // Update border color on all edges
        *border = BorderColor::all(border_color);

        // Update label color(s)
        for child in children.iter() {
            if let Ok(mut color) = q_text.get_mut(child) {
                if color.0 != text_color {
                    color.0 = text_color;
                }
            }
        }
    }
}

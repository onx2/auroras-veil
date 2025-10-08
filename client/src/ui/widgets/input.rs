use bevy::prelude::*;
use bevy::ui::BorderRadius;
use bevy_immediate::ui::ImplCapsUi;
use bevy_immediate::{CapSet, Imm};
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputPlugin, TextInputSettings,
    TextInputSystem, TextInputTextColor, TextInputTextFont, TextInputValue,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TextInputPlugin);
    app.add_systems(Update, apply_input_look.after(TextInputSystem));
}

/// Marker placed on the text input entity to drive visual updates.
#[derive(Component, Clone)]
pub struct UiTextInput {
    pub disabled: bool,
    pub style: InputStyle,
}

/// Visual style for the input field.
#[derive(Clone)]
pub struct InputStyle {
    pub bg: Color,
    pub bg_hover: Color,
    pub bg_disabled: Color,

    pub border: Color,
    pub border_hover: Color,
    pub border_focus: Color,

    pub text: Color,
    pub text_disabled: Color,

    pub border_px: f32,
    pub radius_px: f32,
    pub font_size: f32,
}

impl InputStyle {
    pub fn from_theme(theme: &crate::ui::theme::Theme) -> Self {
        Self {
            bg: theme.panel_bg,
            bg_hover: theme.button_bg_hover,
            bg_disabled: theme.button_bg_disabled,

            border: theme.panel_border,
            border_hover: theme.panel_border,
            border_focus: theme.focus_outline,

            text: theme.text_primary,
            text_disabled: theme.text_muted,

            border_px: theme.border_px,
            radius_px: theme.radius,
            font_size: theme.label,
        }
    }
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            // Backgrounds (aligned to panel/button look)
            bg: Color::srgb(0.16, 0.12, 0.08),
            bg_hover: Color::srgb(0.20, 0.15, 0.10),
            bg_disabled: Color::srgba(0.16, 0.12, 0.08, 0.6),

            // Borders (hover slightly brighter, focus uses theme's focus accent)
            border: Color::srgb(0.36, 0.28, 0.20),
            border_hover: Color::srgb(0.44, 0.34, 0.24),
            border_focus: Color::srgb(0.92, 0.86, 0.62),

            // Text colors (primary + muted tone for disabled)
            text: Color::srgb(0.86, 0.80, 0.58),
            text_disabled: Color::srgb(0.66, 0.60, 0.44),

            // Shape/typography
            border_px: 2.0,
            radius_px: 2.0, // matches the button default radius
            font_size: 28.0,
        }
    }
}

/// Builder-style props for creating a text input.
#[derive(Clone)]
pub struct InputProps {
    pub width: Val,
    pub height: Val,
    pub padding: UiRect,

    pub disabled: bool,
    /// If true, starts unfocused/inactive (cursor hidden) until focused.
    pub inactive: bool,

    /// Placeholder text shown when empty and inactive.
    pub placeholder: String,

    /// If true, do not clear text on submit.
    pub retain_on_submit: bool,
    /// Optional masking (e.g., Some('•')) to hide input text.
    pub mask_character: Option<char>,

    pub style: InputStyle,
}

impl Default for InputProps {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),

            disabled: false,
            inactive: false,

            placeholder: String::new(),
            retain_on_submit: false,
            mask_character: None,

            style: InputStyle::default(),
        }
    }
}

impl InputProps {
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
    pub fn style(mut self, style: InputStyle) -> Self {
        self.style = style;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn inactive(mut self, inactive: bool) -> Self {
        self.inactive = inactive;
        self
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }
    pub fn retain_on_submit(mut self, retain: bool) -> Self {
        self.retain_on_submit = retain;
        self
    }
    pub fn mask(mut self, ch: Option<char>) -> Self {
        self.mask_character = ch;
        self
    }
}

/// Spawn a themed, styled single-line text input with a stable identifier.
///
/// - Visuals and interaction mirror the button's sepia/gold theme.
/// - Hover outlines brighten; focus outline uses the theme accent.
/// - The `TextInputPlugin` must be in the app; this module's `plugin(app)` adds it.
///
/// Example usage inside an immediate UI build:
/// ```ignore
/// text_input(ui, ("name_field", 0), InputProps::default().placeholder("Character name"));
/// // Listen for TextInputSubmitMessage in your own system to handle submit events.
/// ```
pub fn text_input<Caps, Id>(ui: &mut Imm<Caps>, id: Id, props: InputProps)
where
    Caps: CapSet + ImplCapsUi,
    Id: Clone + std::hash::Hash + Eq + Send + Sync + 'static,
{
    let style = props.style.clone();
    let placeholder_text = props.placeholder.clone();
    let retain_on_submit = props.retain_on_submit;
    let mask_character = props.mask_character;

    ui.ch_id(id).on_spawn_insert(move || {
        (
            // The text input driving component
            TextInput,
            // Container node
            Node {
                width: props.width,
                height: props.height,
                padding: props.padding,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(style.border_px)),
                ..default()
            },
            BackgroundColor(if props.disabled {
                style.bg_disabled
            } else {
                style.bg
            }),
            TextInputValue("".into()),
            BorderColor::all(style.border),
            BorderRadius::all(Val::Px(style.radius_px)),
            // Text look
            TextInputTextFont(TextFont {
                font: Handle::<Font>::default(),
                font_size: style.font_size,
                ..default()
            }),
            TextInputTextColor(TextColor(if props.disabled {
                style.text_disabled
            } else {
                style.text
            })),
            // Behavior
            TextInputSettings {
                retain_on_submit,
                mask_character,
            },
            TextInputPlaceholder {
                value: placeholder_text,
                // Use the same font & size as the input text
                text_font: Some(TextFont {
                    font: Handle::<Font>::default(),
                    font_size: style.font_size,
                    ..default()
                }),
                // If None, the plugin uses 25% alpha of the input text color.
                // Keep None so placeholder tracks theme text color automatically.
                text_color: None,
            },
            // Start inactive if requested or when disabled.
            TextInputInactive(props.inactive || props.disabled),
            // Our own marker + visual style for hover/focus updates
            UiTextInput {
                disabled: props.disabled,
                style,
            },
        )
    });
}

/// System that applies hover/focus/disabled visuals for all UiTextInput fields.
fn apply_input_look(
    mut q: Query<(
        &UiTextInput,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut TextInputInactive,
        &mut TextInputTextColor,
    )>,
) {
    for (input, interaction, mut bg, mut border, mut inactive, mut text_color) in q.iter_mut() {
        let s = &input.style;

        // Allow focusing the input by clicking it (unless disabled).
        if !input.disabled && matches!(*interaction, Interaction::Pressed) && inactive.0 {
            inactive.0 = false;
        }

        // Choose colors based on state.
        let (bg_color, border_color, text_col) = if input.disabled {
            (s.bg_disabled, s.border, s.text_disabled)
        } else if !inactive.0 {
            // Focused input: emphasize border with focus accent
            (s.bg, s.border_focus, s.text)
        } else {
            match *interaction {
                Interaction::Pressed | Interaction::Hovered => (s.bg_hover, s.border_hover, s.text),
                Interaction::None => (s.bg, s.border, s.text),
            }
        };

        if bg.0 != bg_color {
            bg.0 = bg_color;
        }
        *border = BorderColor::all(border_color);

        if text_color.0.0 != text_col {
            text_color.0 = TextColor(text_col);
        }
    }
}

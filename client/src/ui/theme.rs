use bevy::prelude::*;

/// A simple token-based color system for UI styling.
///
/// Widgets (like buttons) set their visuals by inserting `ThemeBackgroundColor`
/// and/or `ThemeFontColor` components with a token. Systems in this module then
/// translate those tokens into concrete `BackgroundColor` and `TextColor` values
/// using the active `AvUiTheme` resource.
///
/// This mirrors the "design tokens" approach and keeps widget logic decoupled
/// from the exact palette values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
#[reflect(Default)]
pub enum ThemeColorToken {
    #[default]
    Transparent,

    // Generic text colors
    Text,
    TextMuted,
    TextDisabled,

    // Generic surfaces
    Surface,
    SurfaceRaised,

    // Button (normal)
    ButtonBg,
    ButtonBgHover,
    ButtonBgPressed,
    ButtonBgDisabled,
    ButtonText,
    ButtonTextDisabled,

    // Button (primary / call-to-action)
    ButtonPrimaryBg,
    ButtonPrimaryBgHover,
    ButtonPrimaryBgPressed,
    ButtonPrimaryBgDisabled,
    ButtonPrimaryText,
    ButtonPrimaryTextDisabled,
}

/// Public constants for ergonomic use in other modules.
///
/// Example:
/// commands.entity(e).insert(ThemeBackgroundColor(tokens::BUTTON_BG));
pub mod tokens {
    use super::ThemeColorToken;

    // Normal button
    pub const BUTTON_BG: ThemeColorToken = ThemeColorToken::ButtonBg;
    pub const BUTTON_BG_HOVER: ThemeColorToken = ThemeColorToken::ButtonBgHover;
    pub const BUTTON_BG_PRESSED: ThemeColorToken = ThemeColorToken::ButtonBgPressed;
    pub const BUTTON_BG_DISABLED: ThemeColorToken = ThemeColorToken::ButtonBgDisabled;
    pub const BUTTON_TEXT: ThemeColorToken = ThemeColorToken::ButtonText;
    pub const BUTTON_TEXT_DISABLED: ThemeColorToken = ThemeColorToken::ButtonTextDisabled;

    // Primary button
    pub const BUTTON_PRIMARY_BG: ThemeColorToken = ThemeColorToken::ButtonPrimaryBg;
    pub const BUTTON_PRIMARY_BG_HOVER: ThemeColorToken = ThemeColorToken::ButtonPrimaryBgHover;
    pub const BUTTON_PRIMARY_BG_PRESSED: ThemeColorToken = ThemeColorToken::ButtonPrimaryBgPressed;
    pub const BUTTON_PRIMARY_BG_DISABLED: ThemeColorToken =
        ThemeColorToken::ButtonPrimaryBgDisabled;
    pub const BUTTON_PRIMARY_TEXT: ThemeColorToken = ThemeColorToken::ButtonPrimaryText;
    pub const BUTTON_PRIMARY_TEXT_DISABLED: ThemeColorToken =
        ThemeColorToken::ButtonPrimaryTextDisabled;

    // Common text
    pub const TEXT: ThemeColorToken = ThemeColorToken::Text;
    pub const TEXT_MUTED: ThemeColorToken = ThemeColorToken::TextMuted;
    pub const TEXT_DISABLED: ThemeColorToken = ThemeColorToken::TextDisabled;

    // Surfaces
    pub const SURFACE: ThemeColorToken = ThemeColorToken::Surface;
    pub const SURFACE_RAISED: ThemeColorToken = ThemeColorToken::SurfaceRaised;
}

/// Attach this to an entity to request a themed background color.
/// A system will translate the token into a concrete `BackgroundColor`.
#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ThemeBackgroundColor(pub ThemeColorToken);

/// Attach this to an entity to request a themed font color.
/// A system will translate the token into a concrete `TextColor`.
#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ThemeFontColor(pub ThemeColorToken);

/// Marker for text that should use the theme's default text color.
#[derive(Component, Default, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct ThemedText;

/// Re-export tokens for convenient `use crate::ui::theme::*` imports.
pub use tokens::*;

/// Simple theme resource that maps tokens to colors.
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct AvUiTheme {
    /// The color used for a subtle gold text, inspired by the provided mockup.
    pub text: Color,
    pub text_muted: Color,
    pub text_disabled: Color,

    /// Core surfaces
    pub surface: Color,
    pub surface_raised: Color,

    // Normal button palette
    pub button_bg: Color,
    pub button_bg_hover: Color,
    pub button_bg_pressed: Color,
    pub button_bg_disabled: Color,
    pub button_text: Color,
    pub button_text_disabled: Color,

    // Primary button palette
    pub button_primary_bg: Color,
    pub button_primary_bg_hover: Color,
    pub button_primary_bg_pressed: Color,
    pub button_primary_bg_disabled: Color,
    pub button_primary_text: Color,
    pub button_primary_text_disabled: Color,
}

impl AvUiTheme {
    /// Returns a color for the given token.
    pub fn color(&self, token: ThemeColorToken) -> Color {
        match token {
            ThemeColorToken::Transparent => Color::NONE,

            ThemeColorToken::Text => self.text,
            ThemeColorToken::TextMuted => self.text_muted,
            ThemeColorToken::TextDisabled => self.text_disabled,

            ThemeColorToken::Surface => self.surface,
            ThemeColorToken::SurfaceRaised => self.surface_raised,

            ThemeColorToken::ButtonBg => self.button_bg,
            ThemeColorToken::ButtonBgHover => self.button_bg_hover,
            ThemeColorToken::ButtonBgPressed => self.button_bg_pressed,
            ThemeColorToken::ButtonBgDisabled => self.button_bg_disabled,
            ThemeColorToken::ButtonText => self.button_text,
            ThemeColorToken::ButtonTextDisabled => self.button_text_disabled,

            ThemeColorToken::ButtonPrimaryBg => self.button_primary_bg,
            ThemeColorToken::ButtonPrimaryBgHover => self.button_primary_bg_hover,
            ThemeColorToken::ButtonPrimaryBgPressed => self.button_primary_bg_pressed,
            ThemeColorToken::ButtonPrimaryBgDisabled => self.button_primary_bg_disabled,
            ThemeColorToken::ButtonPrimaryText => self.button_primary_text,
            ThemeColorToken::ButtonPrimaryTextDisabled => self.button_primary_text_disabled,
        }
    }
}

/// Create the default Aurora's Veil theme palette.
///
/// Colors are approximations from the reference image:
/// - Deep, warm browns for surfaces/buttons
/// - Desaturated gold for text
pub fn create_default_theme() -> AvUiTheme {
    // Helper to create a color from sRGB hex.
    let hex = |rgb: u32| -> Color {
        let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
        let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
        let b = (rgb & 0xFF) as f32 / 255.0;
        Color::srgb(r, g, b)
    };

    // Palette picks (approximate):
    // Base surface: very dark brown
    let surface = hex(0x20170F); // #20170F
    let surface_raised = hex(0x2A2017); // slightly lighter, #2A2017

    // Text: warm desaturated golds
    let text = hex(0xD8BD84); // primary text gold
    let text_muted = hex(0xBFA474);
    let text_disabled = hex(0x8E7C5C);

    // Normal button surface series (muted)
    let button_bg = hex(0x3A2D21);
    let button_bg_hover = hex(0x4A3A2A);
    let button_bg_pressed = hex(0x2D231A);
    let button_bg_disabled = hex(0x2A2118);

    // Primary button surface series (prominent)
    let button_primary_bg = hex(0x5A4432);
    let button_primary_bg_hover = hex(0x6B503A);
    let button_primary_bg_pressed = hex(0x4B3A2B);
    let button_primary_bg_disabled = hex(0x3F3226);

    AvUiTheme {
        text,
        text_muted,
        text_disabled,
        surface,
        surface_raised,

        button_bg,
        button_bg_hover,
        button_bg_pressed,
        button_bg_disabled,
        button_text: text,
        button_text_disabled: text_disabled,

        button_primary_bg,
        button_primary_bg_hover,
        button_primary_bg_pressed,
        button_primary_bg_disabled,
        button_primary_text: text,
        button_primary_text_disabled: text_disabled,
    }
}

/// Apply ThemeBackgroundColor changes to actual BackgroundColor.
fn apply_theme_background_color_changed(
    theme: Res<AvUiTheme>,
    mut commands: Commands,
    mut q: Query<
        (Entity, &ThemeBackgroundColor, Option<&mut BackgroundColor>),
        Changed<ThemeBackgroundColor>,
    >,
) {
    for (entity, token, bg) in &mut q {
        let color = theme.color(**token);
        if let Some(mut bg) = bg {
            bg.0 = color;
        } else {
            commands.entity(entity).insert(BackgroundColor(color));
        }
    }
}

/// Apply ThemeFontColor changes to actual TextColor.
fn apply_theme_font_color_changed(
    theme: Res<AvUiTheme>,
    mut commands: Commands,
    mut q: Query<(Entity, &ThemeFontColor, Option<&mut TextColor>), Changed<ThemeFontColor>>,
) {
    for (entity, token, tc) in &mut q {
        let color = theme.color(**token);
        if let Some(mut tc) = tc {
            tc.0 = color;
        } else {
            commands.entity(entity).insert(TextColor(color));
        }
    }
}

/// When the theme resource changes, re-apply all token-driven colors.
fn refresh_all_theme_colors_on_theme_change(
    theme: Res<AvUiTheme>,
    mut q_bg: Query<&mut BackgroundColor, With<ThemeBackgroundColor>>,
    q_bg_tokens: Query<&ThemeBackgroundColor>,
    mut q_text: Query<&mut TextColor, With<ThemeFontColor>>,
    q_text_tokens: Query<&ThemeFontColor>,
) {
    if !theme.is_changed() {
        return;
    }

    // Update all backgrounds
    for (mut bg, token) in q_bg.iter_mut().zip(q_bg_tokens.iter()) {
        bg.0 = theme.color(**token);
    }

    // Update all text colors
    for (mut tc, token) in q_text.iter_mut().zip(q_text_tokens.iter()) {
        tc.0 = theme.color(**token);
    }
}

/// Automatically apply default text color to any entity with ThemedText and Text.
fn attach_default_text_color_on_themed_text_added(
    mut commands: Commands,
    q: Query<Entity, (Added<ThemedText>, With<Text>, Without<ThemeFontColor>)>,
) {
    for e in &q {
        commands.entity(e).insert(ThemeFontColor(tokens::TEXT));
    }
}

/// Minimal theme plugin. Adds the theme resource and systems that translate
/// tokens into concrete Bevy UI colors.
pub fn plugin(app: &mut App) {
    app.register_type::<ThemeColorToken>()
        .register_type::<ThemeBackgroundColor>()
        .register_type::<ThemeFontColor>()
        .register_type::<ThemedText>()
        .register_type::<AvUiTheme>();

    // Only insert a default theme if the app doesn't provide one.
    if app.world().get_resource::<AvUiTheme>().is_none() {
        app.insert_resource(create_default_theme());
    }

    app.add_systems(
        Update,
        (
            apply_theme_background_color_changed,
            apply_theme_font_color_changed,
            refresh_all_theme_colors_on_theme_change,
            attach_default_text_color_on_themed_text_added,
        ),
    );
}

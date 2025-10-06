use bevy::prelude::*;

/// Theme resource that centralizes palette, spacing, and typography for UI widgets.
/// Keep widget visuals consistent by reading colors/sizes from this resource instead
/// of hard-coding values in individual components.
///
/// Add this module’s `plugin(app)` from `ui::plugin` so the theme is always present.
#[derive(Resource, Clone, Debug)]
pub struct Theme {
    // Base surfaces
    pub bg: Color,           // App background
    pub panel_bg: Color,     // Panel/container background
    pub panel_border: Color, // Panel borders and separators
    pub separator: Color,    // Thin separators/dividers

    // Text
    pub text_primary: Color, // Primary text (titles/labels)
    pub text_muted: Color,   // Secondary/placeholder text

    // Accents
    pub accent: Color,        // Decorative accents, headers
    pub focus_outline: Color, // Focus/selection outlines

    // Buttons
    pub button_bg: Color,
    pub button_bg_hover: Color,
    pub button_bg_active: Color,
    pub button_bg_disabled: Color,
    pub button_text: Color,

    // Layout/typography
    pub radius: f32,    // Default corner radius
    pub border_px: f32, // Default border thickness
    pub separator_px: f32,
    pub pad: f32,   // Default padding
    pub gap: f32,   // Default gap between items
    pub label: f32, // Default label font size
}

impl Theme {
    /// Convenience for setting the global `ClearColor` to the theme background.
    pub fn as_clear_color(&self) -> ClearColor {
        ClearColor(self.bg)
    }
}

impl Default for Theme {
    fn default() -> Self {
        // Sepia/gold palette approximated from the provided UI reference.
        Self {
            // Base surfaces
            bg: Color::srgb(0.12, 0.09, 0.06),
            panel_bg: Color::srgb(0.16, 0.12, 0.08),
            panel_border: Color::srgb(0.36, 0.28, 0.20),
            separator: Color::srgb(0.28, 0.22, 0.16),

            // Text
            text_primary: Color::srgb(0.86, 0.80, 0.58),
            text_muted: Color::srgb(0.72, 0.66, 0.48),

            // Accents
            accent: Color::srgb(0.90, 0.82, 0.62),
            focus_outline: Color::srgb(0.92, 0.86, 0.62),

            // Buttons (close to the "CREATE" button look)
            button_bg: Color::srgb(0.34, 0.26, 0.16),
            button_bg_hover: Color::srgb(0.42, 0.32, 0.20),
            button_bg_active: Color::srgb(0.48, 0.36, 0.22),
            button_bg_disabled: Color::srgb(0.24, 0.18, 0.12),
            button_text: Color::srgb(0.88, 0.82, 0.64),

            // Layout/typography
            radius: 6.0,
            border_px: 2.0,
            separator_px: 1.0,
            pad: 12.0,
            gap: 8.0,
            label: 28.0,
        }
    }
}

/// Minimal plugin used by `ui::plugin` to ensure a `Theme` resource exists.
/// Idempotent: if a Theme is already present, it is not overwritten.
pub(super) fn plugin(app: &mut App) {
    // Insert the default theme if not already present
    if app.world().get_resource::<Theme>().is_none() {
        app.insert_resource(Theme::default());
    }

    // Optionally set clear color to theme background if it hasn't been set by the app.
    let bg = app
        .world()
        .get_resource::<Theme>()
        .map(|t| t.bg)
        .unwrap_or(Color::BLACK);

    if app.world().get_resource::<ClearColor>().is_none() {
        app.insert_resource(ClearColor(bg));
    }
}

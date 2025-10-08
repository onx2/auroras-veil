use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_immediate::ui::ImplCapsUi;
use bevy_immediate::{CapSet, Imm};

use crate::ui::widgets::button::{ButtonProps, button};

/// Visual style for the window.
///
/// If you have global app styles such as `panel_bg` and `panel_border`,
/// pass them into `bg` and `border` here via `WindowProps::style(...)`.
#[derive(Clone)]
pub struct WindowStyle {
    pub bg: Color,
    pub border: Color,
    pub border_px: f32,
    pub radius_px: f32,

    pub title_color: Color,
    pub title_font_size: f32,
}

impl Default for WindowStyle {
    fn default() -> Self {
        Self {
            // Defaults are intentionally neutral; override with your own theme.
            // Map your global `panel_bg` and `panel_border` to these fields.
            bg: Color::srgba(0.10, 0.10, 0.12, 0.98),
            border: Color::srgb(0.35, 0.35, 0.42),
            border_px: 2.0,
            radius_px: 4.0,

            title_color: Color::srgb(0.90, 0.90, 0.92),
            title_font_size: 28.0,
        }
    }
}

/// Builder-style props to control sizing, padding, layering, and styling.
#[derive(Clone)]
pub struct WindowProps {
    pub width: Val,
    pub height: Val,

    /// Inner paddings
    pub header_padding: UiRect,
    pub body_padding: UiRect,

    /// Z-index for the overlay (Global). Higher draws on top.
    pub z_index: i32,

    /// Visuals for the window shell.
    pub style: WindowStyle,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            width: Val::Px(480.0),
            height: Val::Px(360.0),
            header_padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
            body_padding: UiRect::all(Val::Px(16.0)),
            z_index: 1000,
            style: WindowStyle::default(),
        }
    }
}

impl WindowProps {
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn header_padding(mut self, padding: UiRect) -> Self {
        self.header_padding = padding;
        self
    }
    pub fn body_padding(mut self, padding: UiRect) -> Self {
        self.body_padding = padding;
        self
    }
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }
    pub fn style(mut self, style: WindowStyle) -> Self {
        self.style = style;
        self
    }
}

/// Result of rendering a window for the current frame.
pub struct WindowResult {
    /// True on the frame the header close (X) button was clicked.
    pub close_clicked: bool,
}

/// Render a generic, centered, non-resizable overlay window with:
/// - full-screen overlay container (absolute positioned, centered)
/// - background + border panel
/// - header with title (left) and X close button (right)
/// - body that renders caller-provided children below the header
///
/// Use a stable `id` to keep the window entity consistent across frames.
///
/// Example:
/// let res = window(ui, "inventory", "Inventory", WindowProps::default(), |ui| {
///     ui.ch().on_spawn_text("Body content here");
/// });
/// if res.close_clicked { /* hide or despawn */ }
pub fn window<Caps, Id>(
    ui: &mut Imm<Caps>,
    id: Id,
    title: impl Into<String>,
    props: WindowProps,
    content: impl FnOnce(&mut Imm<Caps>),
) -> WindowResult
where
    Caps: CapSet + ImplCapsUi,
    Id: Clone + std::hash::Hash + Eq + Send + Sync + 'static,
{
    let mut close_clicked = false;
    let title_str = title.into();
    let WindowStyle {
        bg,
        border,
        border_px,
        radius_px,
        title_color,
        title_font_size,
    } = props.style.clone();

    // Overlay root: absolute, full-screen, centered, high z-index so it's on top.
    ui.ch_id(("window_overlay", id.clone()))
        .on_spawn_insert(move || Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Auto,
            top: Val::Px(0.0),
            bottom: Val::Auto,
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .on_spawn_insert(move || ZIndex(props.z_index))
        .on_spawn_insert(|| FocusPolicy::Block)
        .add(|ui| {
            // Window panel: background + border + radius, fixed size.
            ui.ch_id(("window_panel", id.clone()))
                .on_spawn_insert(move || {
                    (
                        Node {
                            flex_direction: FlexDirection::Column,
                            width: props.width,
                            height: props.height,
                            overflow: Overflow::clip(),
                            border: UiRect::all(Val::Px(border_px)),
                            ..default()
                        },
                        BackgroundColor(bg),
                        BorderColor::all(border),
                        BorderRadius::all(Val::Px(radius_px)),
                        FocusPolicy::Block,
                    )
                })
                .add(|ui| {
                    // Header row: title (left) and close button (right)
                    ui.ch_id(("window_header", id.clone()))
                        .on_spawn_insert(move || Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            padding: props.header_padding,
                            ..default()
                        })
                        .add(|ui| {
                            // Title text
                            ui.ch_id(("window_title", id.clone()))
                                .on_spawn_insert(move || {
                                    (
                                        Text(title_str.clone()),
                                        TextFont {
                                            font: Handle::<Font>::default(),
                                            font_size: title_font_size,
                                            ..default()
                                        },
                                        TextColor(title_color),
                                    )
                                });

                            // Close button
                            let close_btn = button(
                                ui,
                                ("window_close_btn", id.clone()),
                                "X",
                                ButtonProps::default()
                                    .size(Val::Px(28.0), Val::Px(28.0))
                                    .padding(UiRect::all(Val::Px(4.0))),
                            );
                            if close_btn.clicked {
                                close_clicked = true;
                            }
                        });

                    // Body content container
                    ui.ch_id(("window_body", id))
                        .on_spawn_insert(move || Node {
                            flex_grow: 1.0,
                            width: percent(100.0),
                            padding: props.body_padding,
                            ..default()
                        })
                        .add(|ui| {
                            content(ui);
                        });
                });
        });

    WindowResult { close_clicked }
}

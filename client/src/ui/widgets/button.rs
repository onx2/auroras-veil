use bevy::ecs::spawn::SpawnableList;
use bevy::prelude::*;

#[derive(Component, Default, Clone, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component, Clone, Default)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub shape: ButtonShape,
}

#[derive(Component, Default, Clone, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component, Clone, Default)]
pub enum ButtonVariant {
    #[default]
    Normal,
    Primary,
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
pub enum ButtonShape {
    #[default]
    Rectangle,
    Square,
}

const BUTTON_SMALL_WIDTH: f32 = 100.0;
const BUTTON_MEDIUM_WIDTH: f32 = 150.0;

impl ButtonProps {
    pub fn get_background_color(&self, interaction: Interaction) -> Color {
        match self.variant {
            ButtonVariant::Normal => match interaction {
                Interaction::None => Color::srgb(0.08, 0.05, 0.02),
                _ => Color::srgb(0.329, 0.255, 0.192),
            },
            ButtonVariant::Primary => match interaction {
                Interaction::None => Color::srgb(0.408, 0.286, 0.173),
                _ => Color::srgb(0.494, 0.365, 0.247),
            },
        }
    }

    pub fn get_width(&self) -> Val {
        match self.shape {
            ButtonShape::Rectangle => match self.size {
                ButtonSize::Small => px(100.0),
                ButtonSize::Medium => px(150.0),
            },
            ButtonShape::Square => match self.size {
                ButtonSize::Small => px(40.0),
                ButtonSize::Medium => px(65.0),
            },
        }
    }

    pub fn get_height(&self) -> Val {
        match self.size {
            ButtonSize::Small => px(40.0),
            ButtonSize::Medium => px(65.0),
        }
    }
}

// Marker to identify all instances of this custom button
#[derive(Component)]
pub struct AvButton;

pub fn button<C: SpawnableList<ChildOf> + Send + Sync + 'static>(
    children: C,
    props: ButtonProps,
) -> impl Bundle {
    (
        Button,
        AvButton,
        Node {
            min_width: props.get_width(),
            height: props.get_height(),
            border: UiRect::all(px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(props.get_background_color(Interaction::None)),
        BorderColor::all(Color::srgb(0.23, 0.15, 0.08)),
        TextColor(Color::srgb(0.89, 0.75, 0.53)),
        Children::spawn(children),
        props,
    )
}

// Simple visual feedback for all AvButton entities.
fn button_interaction_visuals(
    mut q: Query<
        (&Interaction, &mut BackgroundColor, &ButtonProps),
        (Changed<Interaction>, With<AvButton>),
    >,
) {
    for (interaction, mut bg, props) in &mut q {
        *bg = BackgroundColor(props.get_background_color(*interaction));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, button_interaction_visuals);
}

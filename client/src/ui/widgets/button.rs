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
#[derive(Component)]
struct ButtonImages {
    normal: Handle<Image>,
    hovered: Handle<Image>,
    pressed: Handle<Image>,
}

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
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode {
            image: default(),
            ..default()
        },
        TextColor(Color::srgb(0.89, 0.75, 0.53)),
        Children::spawn(children),
        props,
    )
}

fn init_button_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &ButtonProps, &mut ImageNode), (With<AvButton>, Without<ButtonImages>)>,
) {
    for (entity, props, mut image_node) in &mut q {
        let base = match props.shape {
            ButtonShape::Square => "ui/button/SquareButton",
            ButtonShape::Rectangle => "ui/button/SquareButton",
        };
        let normal: Handle<Image> = asset_server.load(format!("{base}_Normal.png"));
        let hovered: Handle<Image> = asset_server.load(format!("{base}_Hovered.png"));
        let pressed: Handle<Image> = asset_server.load(format!("{base}_Pressed.png"));
        image_node.image = normal.clone();
        commands.entity(entity).insert(ButtonImages {
            normal,
            hovered,
            pressed,
        });
    }
}

// Simple visual feedback for all AvButton entities.
fn button_interaction_visuals(
    mut q: Query<
        (&Interaction, &mut ImageNode, &ButtonImages),
        (Changed<Interaction>, With<AvButton>),
    >,
) {
    for (interaction, mut image, images) in &mut q {
        image.image = match *interaction {
            Interaction::Pressed => images.pressed.clone(),
            Interaction::Hovered => images.hovered.clone(),
            Interaction::None => images.normal.clone(),
        };
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (init_button_images, button_interaction_visuals));
}

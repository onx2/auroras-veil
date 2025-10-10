use bevy::picking::PickingSystems;
use bevy::prelude::*;
use bevy::sprite::{BorderRect, SliceScaleMode, TextureSlicer};

use super::{ButtonIcon, ButtonSize};

#[derive(Component, Default, Clone, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component, Clone, Default)]
pub struct IconButtonProps {
    pub size: ButtonSize,
    pub icon: ButtonIcon,
}

impl IconButtonProps {
    pub fn get_size(&self) -> Val {
        match self.size {
            ButtonSize::Small => px(36.0),
            ButtonSize::Medium => px(48.0),
        }
    }

    pub fn get_icon_size(&self) -> Val {
        match self.size {
            ButtonSize::Small => px(24.0),
            ButtonSize::Medium => px(32.0),
        }
    }
}

// Marker to identify all instances of this custom icon button
#[derive(Component)]
pub struct AvIconButton;

#[derive(Component)]
struct AvIconButtonIcon;

pub fn icon_button(props: IconButtonProps) -> impl Bundle {
    (
        Button,
        AvIconButton,
        Node {
            width: props.get_size(),
            height: props.get_size(),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode {
            image: default(), // Set by `update_visuals`
            color: Color::srgba(1., 1., 1., 0.25),
            image_mode: NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect {
                    left: 6.0,
                    right: 6.0,
                    top: 6.0,
                    bottom: 6.0,
                },
                // Tiling for the border
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                // Stretch for the center
                center_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            }),
            ..default()
        },
        children![(
            AvIconButtonIcon,
            ImageNode::default().with_color(Color::srgba(1., 1., 1., 0.5)),
            Node {
                width: props.get_icon_size(),
                height: props.get_icon_size(),
                ..default()
            },
        )],
        props,
    )
}

#[derive(Resource)]
pub struct IconButtonAssets {
    pub bg_normal: Handle<Image>,
    pub bg_hovered: Handle<Image>,
    pub bg_pressed: Handle<Image>,
    pub icon_spells: Handle<Image>,
    pub icon_journal: Handle<Image>,
    pub icon_settings: Handle<Image>,
    pub icon_character: Handle<Image>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        |mut commands: Commands, asset_server: Res<AssetServer>| {
            commands.insert_resource(IconButtonAssets {
                bg_normal: asset_server.load("ui/button/secondary_button_normal.png"),
                bg_hovered: asset_server.load("ui/button/secondary_button_hovered.png"),
                bg_pressed: asset_server.load("ui/button/secondary_button_pressed.png"),
                icon_spells: asset_server.load("ui/button/icons/spells.png"),
                icon_journal: asset_server.load("ui/button/icons/journal.png"),
                icon_settings: asset_server.load("ui/button/icons/settings.png"),
                icon_character: asset_server.load("ui/button/icons/character.png"),
            });
        },
    );

    app.add_systems(PreUpdate, (update_visuals).in_set(PickingSystems::Last));
}

fn update_visuals(
    mut buttons: Query<
        (&Interaction, &mut ImageNode, &IconButtonProps, &Children),
        (Changed<Interaction>, With<AvIconButton>),
    >,
    mut icons: Query<&mut ImageNode, (With<AvIconButtonIcon>, Without<AvIconButton>)>,
    images: Res<IconButtonAssets>,
) {
    for (interaction, mut bg_image, props, children) in &mut buttons {
        bg_image.image = match *interaction {
            Interaction::Pressed => images.bg_pressed.clone(),
            Interaction::Hovered => images.bg_hovered.clone(),
            Interaction::None => images.bg_normal.clone(),
        };

        let Some(child) = children.get(0) else {
            warn!("AvIconButton must always have a child");
            continue;
        };

        let Ok(mut icon_node) = icons.get_mut(*child) else {
            warn!("AvIconButton child must be an ImageNode");
            continue;
        };

        icon_node.image = match props.icon {
            ButtonIcon::Spells => images.icon_spells.clone(),
            ButtonIcon::Journal => images.icon_journal.clone(),
            ButtonIcon::Settings => images.icon_settings.clone(),
            ButtonIcon::Character => images.icon_character.clone(),
        };
    }
}

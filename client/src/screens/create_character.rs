//! The screen state for creating a new character.

use crate::stdb::create_character_reducer::create_character;
use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
    ui::widgets::button::{ButtonProps, button_id},
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_immediate::ui::{CapsUi, ImplCapsUi};
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
};
use bevy_spacetimedb::ReadReducerMessage;

#[derive(Component)]
struct CreateCharacterEntity;

#[derive(Component)]
struct CreateCharacterUiRoot;

#[derive(SystemParam)]
struct CCParams<'w> {
    stdb: SpacetimeDB<'w>,
}

impl ImmediateAttach<CapsUi> for CreateCharacterUiRoot {
    type Params = CCParams<'static>;

    fn construct(ui: &mut Imm<CapsUi>, params: &mut CCParams) {
        // Centered "Create" button styled by the reusable widget.
        let props = ButtonProps::default()
            .size(Val::Px(360.0), Val::Px(72.0))
            .padding(UiRect::axes(Val::Px(20.0), Val::Px(12.0)));

        let res = button_id(ui, "create_btn", "Create", props);
        if res.clicked {
            println!("Screen::CreateCharacter -> create_character");
            if let Err(_) = params.stdb.reducers().create_character("Jeff".into(), 1, 1) {
                println!("Unable to create character due to a networking issue.");
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, CreateCharacterUiRoot>::new());
    app.add_systems(OnEnter(Screen::CreateCharacter), setup);
    app.add_systems(
        Update,
        on_character_create.run_if(in_state(Screen::CreateCharacter)),
    );
    app.add_systems(OnExit(Screen::CreateCharacter), cleanup);
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CreateCharacterEntity>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
}

fn on_character_create(
    mut events: ReadReducerMessage<CreateCharacter>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for event in events.read() {
        println!("CreateCharacter: {:?}", event.result.event.status);
        match event.result.event.status {
            spacetimedb_sdk::Status::Committed => {
                next_screen.set(Screen::CharacterSelect);
            }
            spacetimedb_sdk::Status::Failed(ref msg) => {
                println!("Failed to create character -> Reason: {:?}", msg);
            }
            spacetimedb_sdk::Status::OutOfEnergy => {
                println!("Failed to create character -> Reason: OutOfEnergy");
            }
        }
    }
}

fn setup(mut commands: Commands) {
    println!("Screen::CreateCharacter -> setup");
    commands.spawn((
        CreateCharacterEntity,
        CreateCharacterUiRoot,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));
}

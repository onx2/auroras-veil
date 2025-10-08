//! The screen state for creating a new character.

use crate::stdb::create_character_reducer::create_character;
use crate::ui::widgets::window::WindowProps;
use crate::{
    screens::Screen,
    spacetime::{SpacetimeDB, reducers::CreateCharacter},
    ui::widgets::{
        button::{ButtonProps, button},
        window::window,
    },
};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::{CapsUi, text::ImmUiText},
};
use bevy_spacetimedb::ReadReducerMessage;

#[derive(Component)]
struct CreateCharacterEntity;

#[derive(Resource, Default)]
struct CreateCharacterWindowState {
    open: bool,
}

#[derive(Component)]
struct CreateCharacterUiRoot;

#[derive(SystemParam)]
struct CCParams<'w> {
    stdb: SpacetimeDB<'w>,
    window: ResMut<'w, CreateCharacterWindowState>,
}

impl ImmediateAttach<CapsUi> for CreateCharacterUiRoot {
    type Params = CCParams<'static>;

    fn construct(ui: &mut Imm<CapsUi>, params: &mut CCParams) {
        let props = ButtonProps::default()
            .size(Val::Px(360.0), Val::Px(72.0))
            .padding(UiRect::axes(Val::Px(20.0), Val::Px(12.0)));

        let res = button(ui, "create_character_btn", "Create", props);
        if res.clicked {
            println!("Screen::CreateCharacter -> create_character");
            if let Err(_) = params.stdb.reducers().create_character("Jeff".into(), 1, 1) {
                println!("Unable to create character due to a networking issue.");
            }
        }

        draw_create_character(ui, &mut params.window);
    }
}

pub(super) fn plugin(app: &mut App) {
    // Ensure the window state resource exists before any immediate UI systems run
    app.insert_resource(CreateCharacterWindowState::default());

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

fn setup(mut commands: Commands, mut window_state: ResMut<CreateCharacterWindowState>) {
    println!("Screen::CreateCharacter -> setup");
    // Open the window when entering this screen
    window_state.open = true;

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

fn draw_create_character(
    ui: &mut bevy_immediate::Imm<bevy_immediate::ui::CapsUi>,
    state: &mut CreateCharacterWindowState,
) {
    if !state.open {
        return;
    }
    let props = WindowProps::default().size(Val::Px(480.0), Val::Px(360.0));

    let res = window(
        ui,
        "create_character_window",
        "Create Character",
        props,
        |ui| {
            // Body content goes here
            ui.ch_id(("cc_body", 0))
                .on_spawn_insert(|| Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .add(|ui| {
                    ui.ch().on_spawn_text("Pick a name:");
                    // If you have an input widget, render it here. Placeholder:
                    ui.ch().on_spawn_text("<name input here>");

                    // Buttons row at the bottom-right
                    ui.ch_id(("buttons_row", 0))
                        .on_spawn_insert(|| Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            ..Default::default()
                        })
                        .add(|ui| {
                            let btn_props = ButtonProps::default()
                                .size(Val::Px(120.0), Val::Px(40.0))
                                .padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0)));

                            let create = button(ui, "cc_create_btn", "Create", btn_props.clone());
                            if create.clicked {
                                println!("Create clicked");
                                // call your reducer here if you have access to it
                            }

                            let cancel = button(ui, "cc_cancel_btn", "Cancel", btn_props);
                            if cancel.clicked {
                                println!("Cancel clicked");
                                state.open = false;
                            }
                        });
                });
        },
    );

    if res.close_clicked {
        // Stop emitting next frame to close the window
        state.open = false;
    }
}

use crate::screens::create_character::CreateCharacterForm;
use crate::spacetime::SpacetimeDB;
use crate::stdb::create_character_reducer::create_character;
use crate::ui::widgets::button::{ButtonProps, ButtonVariant, button};
use bevy::{prelude::*, ui_widgets::observe};

pub fn create_button() {
    (
        button(
            Spawn(Text::new("Create")),
            ButtonProps {
                variant: ButtonVariant::Primary,
                ..default()
            },
        ),
        observe(
            |_: On<Pointer<Click>>, stdb: SpacetimeDB, form: Res<CreateCharacterForm>| {
                if let Err(_) =
                    stdb.reducers()
                        .create_character(form.name.clone(), form.race, form.class)
                {
                    println!("Unable to create character due to a networking issue.");
                }
            },
        ),
    );
}

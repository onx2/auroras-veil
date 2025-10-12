mod character;
mod movement;
mod progression;
mod seed;
mod types;

use movement::entity_movement;
use seed::seed_static_data;
use spacetimedb::{Identity, ReducerContext, Table, table};
use types::*;

use crate::character::{CharacterPawn, character_def, character_pawn};

#[table(name = player)]
struct Player {
    #[primary_key]
    identity: Identity,
}

#[table(name = transform, public)]
pub struct Transform {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    /// Position of the entity. In 2d, the last value of the `Vec3` can be used for z-ordering.
    pub translation: Vec3,
    /// Rotation of the entity.
    /// This might not be necssary with root rotation in animations,
    /// unless I need to use this to determine if I'm facing an entity.
    pub rotation: Quat,
    /// Scale of the entity.
    pub scale: Vec3,

    /// Deterministic spatial hashing computed at runtime when transform gets updated.
    /// Used to filter down the transforms required to be processed for spatial filters.
    #[index(btree)]
    pub chunk_id: u32,
}

/// An ephemeral, generic representation of an in-game entity "spawned" into the world.
/// Anything, static or dynamic, should have an individual row in this table if it needs
/// to be represented in the client application.
#[table(name = entity, public)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    #[index(btree)]
    pub transform_id: u32,
}

#[table(name = class, public)]
pub struct Class {
    #[primary_key]
    pub id: u32,

    #[unique]
    pub name: String,

    pub description: String,
}

#[table(name = race, public)]
pub struct Race {
    #[primary_key]
    pub id: u32,

    #[unique]
    pub name: String,

    pub description: String,
}

#[table(name = health, public)]
pub struct Health {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    pub health: u16,
    pub max_health: u16,
}

impl Health {
    pub fn new(max_health: u16) -> Self {
        Self {
            id: 0,
            health: max_health,
            max_health,
        }
    }

    /// Helper function to update health on local copy.
    /// THIS DOES NOT UPDATE THE DATABASE
    pub fn update(&mut self, health: u16) {
        self.health = health.clamp(0, self.max_health);
    }
}

#[table(name = mana, public)]
pub struct Mana {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    pub mana: u16,
    pub max_mana: u16,
}

impl Mana {
    pub fn new(max_mana: u16) -> Self {
        Self {
            id: 0,
            mana: max_mana,
            max_mana,
        }
    }

    /// Helper function to update mana on local copy.
    /// THIS DOES NOT UPDATE THE DATABASE
    pub fn update(&mut self, mana: u16) {
        self.mana = mana.clamp(0, self.max_mana);
    }
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    seed_static_data(ctx);
    movement::init(ctx);
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Err(msg) = leave_world(ctx) {
        log::error!("Unable to leave the world properly: {}", msg);
    }
}

#[spacetimedb::reducer]
pub fn enter_world(ctx: &ReducerContext, character_id: u32) -> Result<(), String> {
    // Does this character exist?
    let Some(character) = ctx.db.character_def().id().find(character_id) else {
        log::warn!(
            "Enter world attempt failed: NotFound\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Invalid character"));
    };

    // Does this player own the character?
    if !character.identity.eq(&ctx.sender) {
        log::warn!(
            "Enter world attempt failed: NotAuthorized\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Invalid character"));
    }

    // Is this character already in the game?
    if let Some(_) = ctx.db.character_pawn().identity().find(ctx.sender) {
        log::warn!(
            "Enter world attempt failed: AlreadyInGame\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Character is already in game"));
    }

    // Get the data necessary for inserting new rows
    let Some(transform) = ctx.db.transform().id().find(character.transform_id) else {
        log::warn!(
            "Enter world attempt failed: InvalidTransform\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Character doesn't have a transform?!"));
    };

    let entity = ctx.db.entity().insert(Entity {
        id: 0,
        transform_id: transform.id,
    });
    ctx.db.character_pawn().insert(CharacterPawn {
        id: 0,
        identity: ctx.sender,
        entity_id: entity.id,
        character_id: character.id,
    });

    Ok(())
}

#[spacetimedb::reducer]
pub fn leave_world(ctx: &ReducerContext) -> Result<(), String> {
    let Some(ci) = ctx.db.character_pawn().identity().find(ctx.sender) else {
        return Err(format!("No valid character instance"));
    };
    ctx.db.character_pawn().identity().delete(ctx.sender);
    ctx.db.entity_movement().entity_id().delete(ci.entity_id);
    ctx.db.entity().id().delete(ci.entity_id);

    Ok(())
}

mod chunk;
mod types;

use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

use crate::types::{Quat, Vec3};

#[spacetimedb::table(name = player)]
pub struct Player {
    #[primary_key]
    identity: Identity,
}

#[spacetimedb::table(name = character, public)]
pub struct Character {
    #[primary_key]
    #[auto_inc]
    id: u32,

    /// Reference to the "owning" player of this character
    #[index(btree)]
    identity: Identity,

    #[index(btree)]
    name: String,

    /// Persistent storage of the character's transform in game
    /// Duplicated on the entity when spawned in but that is ephemeral
    #[index(btree)]
    transform_id: u32,
}

#[spacetimedb::table(name = transform, public)]
pub struct Transform {
    #[primary_key]
    #[auto_inc]
    id: u32,

    /// Position of the entity. In 2d, the last value of the `Vec3` can be used for z-ordering.
    pub translation: Vec3,
    /// Rotation of the entity.
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
#[spacetimedb::table(name = entity, public)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    id: u32,

    #[index(btree)]
    transform_id: u32,
}

#[derive(SpacetimeType)]
pub enum MoveIntent {
    Idle,
    Position(Vec3),
    Entity(u32),
}

/// The intent of dynamic entities to move in game.
/// i.e.) monsters moving around, player's clicking another player to chase and attack
#[spacetimedb::table(name = entity_movement)]
pub struct EntityMovement {
    #[primary_key]
    pub entity_id: u32,

    pub intent: MoveIntent,
}

/// A type-narrowing table for in-game entities that are specifically
/// player-controlled characters.
#[spacetimedb::table(name = character_instance, public)]
pub struct CharacterInstance {
    #[primary_key]
    #[auto_inc]
    id: u32,

    /// Only one character per player is allowed in-game at a time
    #[unique]
    identity: Identity,

    /// The reference to the persistent data store for this character
    #[index(btree)]
    character_id: u32,

    /// The reference to the generic in-game entity for this character
    #[index(btree)]
    entity_id: u32,
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
    // Called everytime a new client connects
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {
    // Called everytime a client disconnects
}

/// Calculate a deterministic spatial hash for the "chunk"
/// that the given coordinates should reside in
pub fn calculate_chunk_id(x: f32, z: f32) -> u32 {
    chunk::encode(x, z)
}

// #[spacetimedb::reducer]
// pub fn request_move(ctx: &ReducerContext, move_intent: MoveIntent) -> Result<(), String> {
//     Ok(())
// }

#[spacetimedb::reducer]
pub fn enter_world(ctx: &ReducerContext, character_id: u32) -> Result<(), String> {
    // Does this character exist?
    let Some(character) = ctx.db.character().id().find(character_id) else {
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
    if let Some(_) = ctx.db.character_instance().identity().find(ctx.sender) {
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
    ctx.db.character_instance().insert(CharacterInstance {
        id: 0,
        identity: ctx.sender,
        entity_id: entity.id,
        character_id: character.id,
    });
    ctx.db.entity_movement().insert(EntityMovement {
        entity_id: entity.id,
        intent: MoveIntent::Idle,
    });

    Ok(())
}

mod seed;
mod tick;
mod types;

use crate::{seed::seed_static_data, tick::init_game_loop, types::*};
use common::chunk;
use spacetimedb::{Identity, ReducerContext, Table};

#[spacetimedb::table(name = player)]
struct Player {
    #[primary_key]
    identity: Identity,
}

#[spacetimedb::table(name = character, public)]
pub struct Character {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    /// Reference to the "owning" player of this character
    #[index(btree)]
    pub identity: Identity,

    #[index(btree)]
    pub name: String,

    /// Persistent storage of the character's transform in game
    /// Duplicated on the entity when spawned in but that is ephemeral
    #[index(btree)]
    pub transform_id: u32,

    #[index(btree)]
    pub class_id: u32,

    #[index(btree)]
    pub race_id: u32,
}

#[spacetimedb::table(name = transform, public)]
pub struct Transform {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

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
    pub id: u32,

    #[index(btree)]
    pub transform_id: u32,
}

/// The intent of dynamic entities to move in game.
/// i.e.) monsters moving around, player's clicking another player to chase and attack
#[spacetimedb::table(name = entity_movement)]
struct EntityMovement {
    #[primary_key]
    entity_id: u32,

    intent: MoveIntent,

    #[index(btree)]
    is_moving: bool,
}

/// A type-narrowing table for in-game entities that are specifically
/// player-controlled characters.
#[spacetimedb::table(name = character_instance, public)]
pub struct CharacterInstance {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    /// Only one character per player is allowed in-game at a time
    #[unique]
    pub identity: Identity,

    /// The reference to the persistent data store for this character
    #[index(btree)]
    pub character_id: u32,

    /// The reference to the generic in-game entity for this character
    #[index(btree)]
    pub entity_id: u32,
}

#[spacetimedb::table(name = class, public)]
pub struct Class {
    #[primary_key]
    pub id: u32,

    #[unique]
    pub name: String,

    pub description: String,
}

#[spacetimedb::table(name = race, public)]
pub struct Race {
    #[primary_key]
    pub id: u32,

    #[unique]
    pub name: String,

    pub description: String,
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    init_game_loop(ctx);
    seed_static_data(ctx);
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Err(msg) = leave_world(ctx) {
        log::error!("Unable to leave the world properly: {}", msg);
    }
}

// #[spacetimedb::reducer]
// pub fn request_move(ctx: &ReducerContext, move_intent: MoveIntent) -> Result<(), String> {
//     // todo:
//     // - calculate new position if necessary
//     //      match move_intent {
//     //          MoveIntent::Idle => {
//     //              // idle is always allow right now
//     //          }
//     //          MoveIntent::Entity(id) => {}
//     //          MoveIntent::Position(translation) => {}
//     //      }
//     // - calculate overlaps ?
//     //   - is this entity dynamic?
//     Ok(())
// }

#[spacetimedb::reducer]
pub fn create_character(
    ctx: &ReducerContext,
    name: String,
    race_id: u32,
    class_id: u32,
) -> Result<(), String> {
    let trimmed_name = name.trim();

    // Is this name valid?
    if trimmed_name.len() == 0 {
        log::warn!(
            "Create character attempt failed: InvalidName\nidentity: {}",
            ctx.sender
        );
        return Err(format!("Invalid character name."));
    }

    // Are the race and class IDs valid?
    if ctx.db.race().id().find(race_id).is_none() {
        return Err(format!("Invalid race."));
    }
    if ctx.db.class().id().find(class_id).is_none() {
        return Err(format!("Invalid class."));
    }

    if ctx
        .db
        .character()
        .iter()
        .filter(|row| row.identity == ctx.sender)
        .count()
        >= 5
    {
        log::warn!(
            "Create character attempt failed: MaxCharacters\nidentity: {}",
            ctx.sender
        );
        return Err(format!("Max characters reached for player."));
    }

    // Is this name taken?
    if ctx
        .db
        .character()
        .iter()
        .filter(|char_row| char_row.name.eq(trimmed_name))
        .next()
        .is_some()
    {
        log::warn!(
            "Create character attempt failed: NameTaken\nidentity: {}",
            ctx.sender
        );
        return Err(format!("Character name is already taken."));
    }

    // todo: what should the default start position be for characters?
    let translation = Vec3::default();
    let chunk_id = chunk::encode(translation.x, translation.z);
    let transform = ctx.db.transform().insert(Transform {
        id: 0,
        translation: translation,
        rotation: Quat::default(),
        scale: Vec3::default(),
        chunk_id: chunk_id,
    });
    ctx.db.character().insert(Character {
        id: 0,
        name: trimmed_name.into(),
        identity: ctx.sender,
        transform_id: transform.id,
        race_id: race_id,
        class_id: class_id,
    });
    // todo: stats?
    Ok(())
}

#[spacetimedb::reducer]
pub fn delete_character(ctx: &ReducerContext, character_id: u32) -> Result<(), String> {
    // Does this character exist?
    let Some(character) = ctx.db.character().id().find(character_id) else {
        log::warn!(
            "Delete character attempt failed: NotFound\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Invalid character"));
    };

    // Does this player own the character?
    if !character.identity.eq(&ctx.sender) {
        log::warn!(
            "Delete character attempt failed: NotAuthorized\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Invalid character"));
    }

    // Is this character already in the game?
    if let Some(_) = ctx.db.character_instance().identity().find(ctx.sender) {
        log::warn!(
            "Delete character attempt failed: InGame\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Cannot delete a character in game."));
    }

    ctx.db.character().delete(character);

    Ok(())
}

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
        is_moving: false,
    });

    Ok(())
}

#[spacetimedb::reducer]
pub fn leave_world(ctx: &ReducerContext) -> Result<(), String> {
    let Some(ci) = ctx.db.character_instance().identity().find(ctx.sender) else {
        return Err(format!("No valid character instance"));
    };
    ctx.db.character_instance().identity().delete(ctx.sender);
    ctx.db.entity_movement().entity_id().delete(ci.entity_id);
    ctx.db.entity().id().delete(ci.entity_id);

    Ok(())
}

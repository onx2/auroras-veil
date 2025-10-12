use crate::{
    Health, Mana, Transform, class, health, mana, race, transform,
    types::{Quat, Vec3},
};
use common::chunk;
use spacetimedb::{
    Filter, Identity, ReducerContext, SpacetimeType, Table, client_visibility_filter, reducer,
    table,
};

const MAX_CHARACTERS_PER_PLAYER: usize = 5;

#[client_visibility_filter]
const CHARACTER_SECURITY: Filter =
    Filter::Sql("SELECT * FROM character_def WHERE identity = :sender");

#[table(name = character_def, public)]
pub struct CharacterDef {
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

    #[index(btree)]
    pub health_id: u32,

    #[index(btree)]
    pub mana_id: u32,
}

/// A type-narrowing table for in-game entities that are specifically player-controlled characters.
#[table(name = character_pawn, public)]
pub struct CharacterPawn {
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

#[derive(SpacetimeType)]
pub struct CreateCharacterInput {
    pub name: String,
    pub class_id: u32,
    pub race_id: u32,
}

#[reducer]
pub fn create_character(ctx: &ReducerContext, input: CreateCharacterInput) -> Result<(), String> {
    let trimmed_name = input.name.trim();

    // Is this name valid?
    if trimmed_name.len() == 0 {
        log::warn!(
            "Create character attempt failed: InvalidName\nidentity: {}",
            ctx.sender
        );
        return Err(format!("Invalid character name."));
    }

    // Are the race and class IDs valid?
    if ctx.db.race().id().find(input.race_id).is_none() {
        return Err(format!("Invalid race."));
    }
    if ctx.db.class().id().find(input.class_id).is_none() {
        return Err(format!("Invalid class."));
    }

    if ctx
        .db
        .character_def()
        .iter()
        .filter(|row| row.identity == ctx.sender)
        .count()
        >= MAX_CHARACTERS_PER_PLAYER
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
        .character_def()
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
    let health = ctx.db.health().insert(Health::new(100));
    let mana = ctx.db.mana().insert(Mana::new(100));
    ctx.db.character_def().insert(CharacterDef {
        id: 0,
        name: trimmed_name.into(),
        identity: ctx.sender,
        transform_id: transform.id,
        race_id: input.race_id,
        class_id: input.class_id,
        health_id: health.id,
        mana_id: mana.id,
    });

    Ok(())
}

#[spacetimedb::reducer]
pub fn delete_character(ctx: &ReducerContext, character_id: u32) -> Result<(), String> {
    // Does this character exist?
    let Some(character) = ctx.db.character_def().id().find(character_id) else {
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
    if let Some(_) = ctx.db.character_pawn().identity().find(ctx.sender) {
        log::warn!(
            "Delete character attempt failed: InGame\nidentity: {}\ncharacter_id: {}",
            ctx.sender,
            character_id
        );
        return Err(format!("Cannot delete a character in game."));
    }

    ctx.db.character_def().delete(character);

    Ok(())
}

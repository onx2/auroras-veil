use crate::{character::character_pawn, entity, transform, types::Vec3};
use common::calculate_step::calculate_step_2d;
use spacetimedb::{ReducerContext, SpacetimeType, Table, TimeDuration, Timestamp, reducer, table};

/// The HZ (FPS) at which the server should tick for movement.
const TICK_RATE: i64 = 30;
const DELTA_MICRO_SECS: i64 = 1_000_000 / TICK_RATE;
const MAX_MOVE_DISTANCE_SQUARED: f32 = 50.0 * 50.0;

#[derive(SpacetimeType)]
pub enum MoveIntent {
    Path(Vec<Vec3>),
    Entity(u32),
}

/// The intent of dynamic entities to move in game.
/// i.e.) monsters moving around, player's clicking another player to chase and attack
#[table(name = entity_movement)]
pub struct EntityMovement {
    #[primary_key]
    pub entity_id: u32,

    pub intent: MoveIntent,
}

#[table(name = movement_tick_timer, scheduled(movement_tick))]
struct MovementTickTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,

    /// Used to compute delta time on server
    last_movement_tick: Timestamp,
}

pub fn init(ctx: &ReducerContext) {
    let movement_tick_interval = TimeDuration::from_micros(DELTA_MICRO_SECS);
    ctx.db.movement_tick_timer().scheduled_id().delete(1);
    ctx.db.movement_tick_timer().insert(MovementTickTimer {
        scheduled_id: 1,
        scheduled_at: spacetimedb::ScheduleAt::Interval(movement_tick_interval),
        last_movement_tick: ctx.timestamp,
    });
}

#[reducer]
fn movement_tick(ctx: &ReducerContext, mut timer: MovementTickTimer) -> Result<(), String> {
    if ctx.sender != ctx.identity() {
        return Err("`movement_tick` may not be invoked by clients.".into());
    }

    // Compute delta time in seconds and update the last movement_tick with the current Timestamp
    let delta_time_secs = ctx
        .timestamp
        .time_duration_since(timer.last_movement_tick)
        .unwrap_or(TimeDuration::from_micros(DELTA_MICRO_SECS))
        .to_micros() as f32
        / 1_000_000.0;
    timer.last_movement_tick = ctx.timestamp;
    ctx.db.movement_tick_timer().scheduled_id().update(timer);

    for mut entity_movement in ctx.db.entity_movement().iter() {
        let Some(se) = ctx.db.entity().id().find(entity_movement.entity_id) else {
            log::warn!("Source entity not found: {}", entity_movement.entity_id);
            ctx.db.entity_movement().delete(entity_movement);
            continue;
        };
        let Some(mut st) = ctx.db.transform().id().find(se.transform_id) else {
            log::warn!("Transform not found for source entity: {}", se.id);
            ctx.db.entity_movement().delete(entity_movement);
            continue;
        };
        match &mut entity_movement.intent {
            MoveIntent::Entity(entity_id) => {
                let Some(te) = ctx.db.entity().id().find(entity_id.clone()) else {
                    log::warn!("Target entity not found: {}", entity_id);
                    ctx.db.entity_movement().delete(entity_movement);
                    continue;
                };
                let Some(tt) = ctx.db.transform().id().find(te.transform_id) else {
                    log::warn!("Transform not found for target entity: {}", entity_id);
                    ctx.db.entity_movement().delete(entity_movement);
                    continue;
                };

                let step = calculate_step_2d(
                    st.translation.to_2d_array(),
                    tt.translation.to_2d_array(),
                    0.5,
                    5.0,
                    delta_time_secs,
                );
                st.translation.x = step.new_position[0];
                st.translation.z = step.new_position[1];
                ctx.db.transform().id().update(st);

                if step.movement_finished {
                    ctx.db.entity_movement().delete(entity_movement);
                }
            }
            MoveIntent::Path(translations) => {
                // get first translation, if we can't it must be empty so delete row
                let Some(tt) = translations.first() else {
                    ctx.db.entity_movement().delete(entity_movement);
                    continue;
                };
                // calculate step toward it
                let step = calculate_step_2d(
                    st.translation.to_2d_array(),
                    tt.to_2d_array(),
                    0.5,
                    5.0,
                    delta_time_secs,
                );

                st.translation.x = step.new_position[0];
                st.translation.z = step.new_position[1];
                ctx.db.transform().id().update(st);

                // if we have reached the point (within acceptance radius), remove that element from the vec
                if step.movement_finished {
                    translations.remove(0);
                }

                // if we have emptied the translations, delete the entry in the table for the entiy movement
                if translations.is_empty() {
                    ctx.db.entity_movement().delete(entity_movement);
                } else if step.movement_finished {
                    ctx.db.entity_movement().entity_id().update(entity_movement);
                }
            }
        }
    }
    Ok(())
}

/// Used by players to request movement.
/// TBD on how monsters and NPCs move, this may be reusable but would need to change a bit.
#[reducer]
pub fn request_move(ctx: &ReducerContext, move_intent: MoveIntent) -> Result<(), String> {
    let Some(character_pawn) = ctx.db.character_pawn().identity().find(ctx.sender) else {
        let err = format!("request_move: unable to find character pawn for sender.");
        log::warn!("{err}");
        return Err(err);
    };

    let Some(character_entity) = ctx.db.entity().id().find(character_pawn.entity_id) else {
        let err = format!("request_move: unable to find entity for character pawn.");
        log::warn!("{err}");
        return Err(err);
    };

    let Some(character_transform) = ctx.db.transform().id().find(character_entity.transform_id)
    else {
        let err = format!("request_move: unable to find chunk for entity.");
        log::warn!("{err}");
        return Err(err);
    };
    let char_translation = character_transform.translation;

    match &move_intent {
        MoveIntent::Entity(entity_id) => {
            if character_pawn.entity_id == *entity_id {
                let err = format!("request_move: cannot move toward yourself.");
                log::warn!("{err}");
                return Err(err);
            }

            let Some(target_entity) = ctx.db.entity().id().find(entity_id) else {
                let err = format!("request_move: target entity not found.");
                log::warn!("{err}");
                return Err(err);
            };

            let Some(target_transform) = ctx.db.transform().id().find(target_entity.transform_id)
            else {
                let err = format!("request_move: target transform not found.");
                log::warn!("{err}");
                return Err(err);
            };
            let target_translation = target_transform.translation;

            let distance_squared = common::distance_squared(
                [target_translation.x, target_translation.z],
                [char_translation.x, char_translation.z],
            );

            if distance_squared >= MAX_MOVE_DISTANCE_SQUARED {
                let err = format!("request_move: target entity is too far away.");
                log::warn!("{err}");
                return Err(err);
            }
        }
        MoveIntent::Path(translations) => {
            let out_of_range = translations.iter().any(|target_translation| {
                let distance_squared = common::distance_squared(
                    [target_translation.x, target_translation.z],
                    [char_translation.x, char_translation.z],
                );
                distance_squared >= MAX_MOVE_DISTANCE_SQUARED
            });

            if out_of_range {
                let err = format!("request_move: translation isn't within range.");
                log::warn!("{err}");
                return Err(err);
            }
        }
    }

    ctx.db.entity_movement().insert(EntityMovement {
        entity_id: character_entity.id,
        intent: move_intent,
    });

    Ok(())
}

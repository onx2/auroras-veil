use spacetimedb::{ReducerContext, TimeDuration, Timestamp};

/// The HZ (FPS) at which the server should tick.
pub(crate) const TICK_RATE: i64 = 30;
pub(crate) const DELTA_MICRO_SECS: i64 = 1_000_000 / TICK_RATE;

#[spacetimedb::table(name = tick_timer, scheduled(tick))]
pub struct TickTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,

    pub last_tick: Timestamp,
}

#[spacetimedb::reducer]
pub fn tick(ctx: &ReducerContext, mut timer: TickTimer) -> Result<(), String> {
    // Game loop schedule can only be invoked by the scheduler
    if ctx.sender != ctx.identity() {
        return Err(
            "Reducer `scheduled` may not be invoked by clients, only via scheduling.".into(),
        );
    }

    // Compute delta time in seconds and update the last tick with the current Timestamp
    let delta_time_secs = ctx
        .timestamp
        .time_duration_since(timer.last_tick)
        .unwrap_or(TimeDuration::from_micros(DELTA_MICRO_SECS))
        .to_micros() as f32
        / 1_000_000.0;
    timer.last_tick = ctx.timestamp;
    ctx.db.tick_timer().scheduled_id().update(timer);

    // handle: movement, collision, etc...
    log::info!("Tick -> delta time: {}", delta_time_secs);
    Ok(())
}

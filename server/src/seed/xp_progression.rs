use crate::progression::{MAX_LEVEL, XpProgression, xp_progression};
use spacetimedb::{ReducerContext, Table};

pub fn seed(ctx: &ReducerContext) {
    if ctx.db.xp_progression().iter().next().is_none() {
        let mut total_xp: u32 = 0;

        for level in 1..=MAX_LEVEL {
            let (base_xp, exponent) = match level {
                1 => (0.0, 1.0),
                2..=10 => (100.0, 1.2),
                11..=20 => (200.0, 1.4),
                21..=30 => (300.0, 1.6),
                31..=40 => (400.0, 1.8),
                41..=50 => (500.0, 2.0),
                _ => unreachable!("Max level is 50"),
            };

            let xp_to_level = ((level as f64).powf(exponent) * base_xp).round() as u32;
            total_xp += xp_to_level;

            ctx.db.xp_progression().insert(XpProgression {
                level,
                total_xp: total_xp,
            });
        }
    }
}

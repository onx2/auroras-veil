//! A module used to define the static data in the database and provides a function
//! to seed that data on database start up.

mod class;
mod race;
mod xp_progression;

use spacetimedb::ReducerContext;

pub fn seed_static_data(ctx: &ReducerContext) {
    race::seed(ctx);
    class::seed(ctx);
    xp_progression::seed(ctx);
}

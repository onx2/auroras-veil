use spacetimedb::table;

pub(crate) const MAX_LEVEL: u8 = 50;

/// Represents the current progression of a player.
/// Used alongside `XpProgression` to compute the player's level and experience percentage.
#[table(name = xp, public)]
pub struct Xp {
    #[primary_key]
    #[auto_inc]
    pub id: u32,

    pub xp: u32,
}

/// The mapping between experience points and levels.
#[table(name = xp_progression, public)]
pub struct XpProgression {
    #[primary_key]
    pub level: u8,

    /// The cumulative experience points required to reach the level.
    pub total_xp: u32,
}

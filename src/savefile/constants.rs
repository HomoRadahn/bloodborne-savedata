/// Where the playtime is stored, in milliseconds
pub const PLAYTIME_OFFSET: usize = 0x08;

/// Distance from the flag AOB
pub const AOB_TO_DEATH_COUNT: usize = 360;
pub const AOB_TO_NEW_GAME_CYCLE: usize = 286;

/// Names are UTF-16, at most 16 units long. The name starts one byte after the offset that was found for it
pub const NAME_START: usize = 1;
pub const NAME_MAX_UNITS: usize = 16;

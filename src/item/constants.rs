/// Goods and armor only use the lower 3 bytes of their id, the highest one is a type tag
pub const ID_MASK: u32 = 0x00FF_FFFF;

/// Type tags, found in the highest byte of the first / second part of a record
pub const TAG_GOOD_FIRST: u8 = 0xB0;
pub const TAG_GOOD_SECOND: u8 = 0x40;
pub const TAG_ARMOR_FIRST: u8 = 0x90;
pub const TAG_ARMOR_SECOND: u8 = 0x10;
pub const TAG_WEAPON_FIRST: u8 = 0x80;
pub const TAG_GEM_FIRST: u8 = 0xC0;

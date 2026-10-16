/// Number of entries in the section, empty ones included
pub const ENTRY_COUNT: usize = 4096;

/// Where the section starts
pub const SECTION_START: usize = 84;

/// The fourth byte of an entry decides what it is
pub const TAG_EMPTY: u8 = 0x00;
pub const TAG_WEAPON: u8 = 0x80;
pub const TAG_ARMOR: u8 = 0x90;
pub const TAG_GEM: u8 = 0xC0;

pub const EMPTY_LENGTH: usize = 8;

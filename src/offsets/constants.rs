pub const APPEARANCE_MAGIC: [u8; 4] = [b'F', b'A', b'C', b'E'];

pub const APPEARANCE_BYTE_COUNT: usize = 0xEB;

pub const LCED_MAGIC: [u8; 4] = [0x4C, 0x43, 0x45, 0x44];

pub const NAME_TO_INVENTORY_OFFSET: usize = 469;

pub const INVENTORY_TO_STORAGE_OFFSET: usize = 0x85DC;

pub const NAME_TO_KEY_OFFSET: usize = 0x7DC9;

pub const NAME_TO_AOB: usize = 68545;

/// Distance from the name to the counters that track how many records the inventory / storage holds
pub const NAME_TO_INVENTORY_COUNTERS: (usize, usize) = (453, 34257);
pub const NAME_TO_STORAGE_COUNTERS: (usize, usize) = (34737, 68541);

pub const KEYS_LENGTH: usize = 2204;

/// In every known save the appearance data comes after this offset
pub const APPEARANCE_SEARCH_START: usize = 0xF000;

/// Source for 1984 slots: https://www.bloodborne-wiki.com/2024/02/full-storage-glitch.html
pub const INVENTORY_SLOTS: usize = 1983;
pub const STORAGE_SLOTS: usize = 1984;

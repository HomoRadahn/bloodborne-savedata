use crate::{gaitem, item::Item};
use constants::*;
use std::io::{self, ErrorKind::InvalidData};

pub(crate) mod constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offsets {
    pub character_name: usize,
    pub inventory: (usize, usize),
    pub storage: (usize, usize),
    pub keys: (usize, usize),
    pub appearance: (usize, usize),
    /// The section with every gem, weapon and armor piece. Start and end (exclusive)
    pub gaitems: (usize, usize),
    pub lced: usize,
}

impl Offsets {
    pub fn build(bytes: &[u8]) -> io::Result<Self> {
        let mut out = Self {
            character_name: 0,
            inventory: (0, 0),
            storage: (0, 0),
            keys: (0, 0),
            appearance: (0, 0),
            gaitems: (0, 0),
            lced: 0,
        };

        if bytes.len() < APPEARANCE_SEARCH_START + APPEARANCE_MAGIC.len() {
            return Err(io::Error::new(InvalidData, "Savedata is too short"));
        }

        let mut last_index = 0;
        for index in APPEARANCE_SEARCH_START..(bytes.len() - 4) {
            if APPEARANCE_MAGIC == bytes[index..index + 4] {
                out.appearance.0 = index + 4;
                out.appearance.1 = out.appearance.0 + APPEARANCE_BYTE_COUNT - 1;
                last_index = index;
                break;
            }
        }

        if out.appearance.0 == 0 {
            return Err(io::Error::new(
                InvalidData,
                "Failed to find appearance data in provided savedata",
            ));
        }

        out.gaitems = (gaitem::constants::SECTION_START, gaitem::end(bytes)?);

        out.inventory.0 = out.appearance.0 - 4 - 34028;
        out.inventory.1 = out.inventory.0 + INVENTORY_SLOTS * Item::LENGTH;

        out.character_name = out.inventory.0 - NAME_TO_INVENTORY_OFFSET;

        out.storage.0 = out.inventory.0 + INVENTORY_TO_STORAGE_OFFSET;
        out.storage.1 = out.storage.0 + STORAGE_SLOTS * Item::LENGTH;

        out.keys.0 = out.character_name + NAME_TO_KEY_OFFSET;
        out.keys.1 = out.keys.0 + KEYS_LENGTH;

        for index in last_index..(bytes.len() - 4) {
            if LCED_MAGIC == bytes[index..index + 4] {
                out.lced = index;
                break;
            }
        }

        if out.lced == 0 {
            return Err(io::Error::new(InvalidData, "Failed to find the LCED marker"));
        }

        Ok(out)
    }
}

impl Offsets {
    /// Absolute offsets of the two counters of the inventory
    pub fn inventory_counters(&self) -> (usize, usize) {
        let (first, second) = NAME_TO_INVENTORY_COUNTERS;

        (self.character_name + first, self.character_name + second)
    }

    /// Absolute offsets of the two counters of the storage
    pub fn storage_counters(&self) -> (usize, usize) {
        let (first, second) = NAME_TO_STORAGE_COUNTERS;

        (self.character_name + first, self.character_name + second)
    }
}

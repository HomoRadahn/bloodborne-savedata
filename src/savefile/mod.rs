use std::{
    fs,
    io::{self, ErrorKind::InvalidInput},
    path::Path,
};

use crate::{
    gaitem::{self, GaItem},
    inventory::Inventory,
    offsets::{Offsets, constants::{APPEARANCE_BYTE_COUNT, NAME_TO_AOB}},
    position::Position,
    stats::Stats,
    util,
};

pub(crate) mod constants;
use constants::*;

pub struct Savefile {
    /// Raw bytes of the savefile as they were read. Changes to the fields below are applied on top of them when writing
    pub bytes: Vec<u8>,
    /// Offsets of different regions in raw bytes, do not modify, unless you know what you are doing
    pub offsets: Offsets,
    /// Death count
    pub death_count: u32,
    /// Which new game cycle the character is in, 0 being the first
    pub new_game_cycle: u8,
    /// Playtime on this profile, in milliseconds
    pub playtime: u32,
    /// Character's name, 16 UTF-16 code units at most
    pub name: String,
    /// Player's position and current map
    pub position: Position,
    /// The player's stats
    pub stats: Stats,
    /// Raw face data of the character
    pub appearance: [u8; APPEARANCE_BYTE_COUNT],
    /// Every gem, weapon and armor piece. Inventories point to these by handle. It always has 4096 entries
    pub gaitems: Vec<GaItem>,
    /// Items and gems held by the player
    pub inventory: Inventory,
    /// Key items held by the player
    pub keys: Inventory,
    /// Items and gems held in the player's storage
    pub storage: Inventory,
}

impl Savefile {
    /// Reads a savefile from a path
    pub fn read(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::from_bytes(fs::read(path)?)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> io::Result<Self> {
        let offsets = Offsets::build(&bytes)?;

        let death_count = util::read_u32(&bytes, offsets.character_name + NAME_TO_AOB + AOB_TO_DEATH_COUNT);
        let new_game_cycle = bytes[offsets.character_name + NAME_TO_AOB + AOB_TO_NEW_GAME_CYCLE];
        let playtime = util::read_u32(&bytes, PLAYTIME_OFFSET);
        let name = Self::read_name(&bytes, &offsets);
        let position = Position::build(&bytes, &offsets)?;
        let stats = Stats::build(&bytes, &offsets);

        let mut appearance = [0; APPEARANCE_BYTE_COUNT];
        appearance.copy_from_slice(&bytes[offsets.appearance.0..=offsets.appearance.1]);

        let gaitems = gaitem::parse(&bytes)?;
        let inventory = Inventory::parse(&bytes, offsets.inventory, Some(offsets.inventory_counters()));
        let keys = Inventory::parse(&bytes, offsets.keys, None);
        let storage = Inventory::parse(&bytes, offsets.storage, Some(offsets.storage_counters()));

        Ok(Self {
            bytes,
            offsets,
            death_count,
            new_game_cycle,
            playtime,
            name,
            position,
            stats,
            appearance,
            gaitems,
            inventory,
            keys,
            storage,
        })
    }

    /// The savefile with every change applied
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut out = self.bytes.clone();

        util::write_u32(&mut out, PLAYTIME_OFFSET, self.playtime);
        util::write_u32(&mut out, self.offsets.character_name + NAME_TO_AOB + AOB_TO_DEATH_COUNT, self.death_count);
        out[self.offsets.character_name + NAME_TO_AOB + AOB_TO_NEW_GAME_CYCLE] = self.new_game_cycle;
        self.write_name(&mut out)?;
        self.position.write(&mut out);
        self.stats.write(&mut out, &self.offsets);
        out[self.offsets.appearance.0..=self.offsets.appearance.1].copy_from_slice(&self.appearance);

        self.inventory.write(&mut out, self.offsets.inventory, Some(self.offsets.inventory_counters()));
        self.keys.write(&mut out, self.offsets.keys, None);
        self.storage.write(&mut out, self.offsets.storage, Some(self.offsets.storage_counters()));

        // Last, as it moves everything after it
        gaitem::write(&mut out, &self.gaitems)?;

        Ok(out)
    }

    /// Writes the savefile to a path
    pub fn write(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, self.to_bytes()?)
    }

    fn read_name(bytes: &[u8], offsets: &Offsets) -> String {
        let start = offsets.character_name + NAME_START;
        let units: Vec<u16> = bytes[start..start + NAME_MAX_UNITS * 2]
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .take_while(|&unit| unit != 0)
            .collect();

        String::from_utf16_lossy(&units)
    }

    fn write_name(&self, bytes: &mut [u8]) -> io::Result<()> {
        let units: Vec<u16> = self.name.encode_utf16().collect();

        if units.is_empty() || units.len() > NAME_MAX_UNITS {
            return Err(io::Error::new(InvalidInput, "Name has to be between 1 and 16 characters long"));
        }

        let start = self.offsets.character_name + NAME_START;
        let field = &mut bytes[start..start + NAME_MAX_UNITS * 2];

        field.fill(0);
        for (pair, unit) in field.chunks_exact_mut(2).zip(units) {
            pair.copy_from_slice(&unit.to_le_bytes());
        }

        Ok(())
    }
}

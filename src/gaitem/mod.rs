//! The section at the start of the savefile that holds every gem, weapon and armor piece the character ever got.
//! Inventories only reference these entries by their handle.

use std::io::{self, ErrorKind::InvalidData};

use crate::{
    gem::Gem,
    slot::{SLOT_COUNT, Slot, SlotShape},
    util,
};

pub(crate) mod constants;
use constants::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Equipment {
    /// Handle, which inventories refer to. The highest byte tells weapons (`0x80`) and armor (`0x90`) apart
    pub handle: u32,
    /// Id of the weapon / armor
    pub id: u32,
    pub durability: u32,
    pub unknown: u32,
    /// Seems to always be 1
    pub slots_marker: u32,
    pub slots: [Slot; SLOT_COUNT],
}

impl Equipment {
    pub(crate) const LENGTH: usize = 60;

    pub fn is_weapon(&self) -> bool {
        (self.handle >> 24) as u8 == TAG_WEAPON
    }

    pub fn is_armor(&self) -> bool {
        (self.handle >> 24) as u8 == TAG_ARMOR
    }

    fn parse(bytes: &[u8], at: usize) -> Self {
        let mut slots = [Slot { shape: SlotShape::Closed, gem: 0 }; SLOT_COUNT];

        for (index, slot) in slots.iter_mut().enumerate() {
            *slot = Slot::parse(bytes, at + 20 + index * Slot::LENGTH);
        }

        Self {
            handle: util::read_u32(bytes, at),
            id: util::read_u32(bytes, at + 4),
            durability: util::read_u32(bytes, at + 8),
            unknown: util::read_u32(bytes, at + 12),
            slots_marker: util::read_u32(bytes, at + 16),
            slots,
        }
    }

    fn write(&self, out: &mut Vec<u8>) {
        for value in [self.handle, self.id, self.durability, self.unknown, self.slots_marker] {
            out.extend_from_slice(&value.to_le_bytes());
        }

        for slot in &self.slots {
            slot.write(out);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GaItem {
    /// Unused entry, with the bytes it had
    Empty([u8; EMPTY_LENGTH]),
    Gem(Gem),
    Equipment(Equipment),
}

impl GaItem {
    pub const EMPTY: Self = Self::Empty([0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]);

    pub fn handle(&self) -> Option<u32> {
        match self {
            Self::Empty(_) => None,
            Self::Gem(gem) => Some(gem.handle),
            Self::Equipment(equipment) => Some(equipment.handle),
        }
    }
}

/// Walks the section, as the length of each entry is only known from its type. Returns where it ends too
fn walk(bytes: &[u8]) -> io::Result<(Vec<(u8, usize)>, usize)> {
    let too_short = || io::Error::new(InvalidData, "Handle section runs past the end of the savedata");

    let mut entries = Vec::with_capacity(ENTRY_COUNT);
    let mut offset = SECTION_START;

    for _ in 0..ENTRY_COUNT {
        let tag = *bytes.get(offset + 3).ok_or_else(too_short)?;
        let length = match tag {
            TAG_EMPTY => EMPTY_LENGTH,
            TAG_GEM => Gem::LENGTH,
            TAG_WEAPON | TAG_ARMOR => Equipment::LENGTH,
            _ => return Err(io::Error::new(InvalidData, format!("Invalid entry {tag:#X} in the handle section"))),
        };

        entries.push((tag, offset));
        offset += length;
    }

    if offset > bytes.len() {
        return Err(too_short());
    }

    Ok((entries, offset))
}

/// Where the section ends
pub(crate) fn end(bytes: &[u8]) -> io::Result<usize> {
    walk(bytes).map(|(_, end)| end)
}

pub(crate) fn parse(bytes: &[u8]) -> io::Result<Vec<GaItem>> {
    Ok(walk(bytes)?
        .0
        .into_iter()
        .map(|(tag, at)| match tag {
            TAG_GEM => GaItem::Gem(Gem::parse(bytes, at)),
            TAG_EMPTY => GaItem::Empty(bytes[at..at + EMPTY_LENGTH].try_into().unwrap()),
            _ => GaItem::Equipment(Equipment::parse(bytes, at)),
        })
        .collect())
}

/// Replaces the section. Entries have different lengths, so what follows moves; the savedata is
/// cut / padded at its end to keep the size
pub(crate) fn write(bytes: &mut Vec<u8>, items: &[GaItem]) -> io::Result<()> {
    if items.len() != ENTRY_COUNT {
        return Err(io::Error::new(InvalidData, "The handle section has to have exactly 4096 entries"));
    }

    let old_end = end(bytes)?;
    let size = bytes.len();
    let mut section = Vec::new();

    for item in items {
        match item {
            GaItem::Empty(raw) => section.extend_from_slice(raw),
            GaItem::Gem(gem) => gem.write(&mut section),
            GaItem::Equipment(equipment) => equipment.write(&mut section),
        }
    }

    bytes.splice(SECTION_START..old_end, section);
    bytes.resize(size, 0);

    Ok(())
}

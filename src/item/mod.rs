use crate::util;

pub(crate) mod constants;
use constants::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Empty,
    Good,
    Weapon,
    Armor,
    Gem,
    /// Not something known, left as is
    Unknown,
}

/// A record of an inventory, storage or key item list, as stored
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    /// Running number of the record
    pub number: u32,
    /// Goods: id with `0xB0` as its highest byte. Weapons, armor, gems: handle of the entry in the handle section
    pub first_part: u32,
    /// Goods: id with `0x40` as its highest byte. Weapons: id. Armor: id with `0x10` as its highest byte. Gems: source
    pub second_part: u32,
    pub amount: u32,
}

impl Item {
    pub(crate) const LENGTH: usize = 16;

    pub const EMPTY: Self = Self { number: 0, first_part: 0, second_part: 0xFFFF_FFFF, amount: 0 };

    pub fn good(id: u32, amount: u32) -> Self {
        Self {
            number: 0,
            first_part: id | (TAG_GOOD_FIRST as u32) << 24,
            second_part: id | (TAG_GOOD_SECOND as u32) << 24,
            amount,
        }
    }

    pub fn weapon(handle: u32, id: u32) -> Self {
        Self { number: 0, first_part: handle, second_part: id, amount: 1 }
    }

    pub fn armor(handle: u32, id: u32) -> Self {
        Self { number: 0, first_part: handle, second_part: id | (TAG_ARMOR_SECOND as u32) << 24, amount: 1 }
    }

    pub fn gem(handle: u32, source: u32) -> Self {
        Self { number: 0, first_part: handle, second_part: source, amount: 1 }
    }

    pub fn kind(&self) -> ItemKind {
        if self.first_part == 0 && self.second_part == Self::EMPTY.second_part && self.amount == 0 {
            return ItemKind::Empty;
        }

        match ((self.first_part >> 24) as u8, (self.second_part >> 24) as u8) {
            (TAG_GOOD_FIRST, TAG_GOOD_SECOND) => ItemKind::Good,
            (TAG_ARMOR_FIRST, TAG_ARMOR_SECOND) => ItemKind::Armor,
            (TAG_WEAPON_FIRST, _) => ItemKind::Weapon,
            (TAG_GEM_FIRST, _) => ItemKind::Gem,
            _ => ItemKind::Unknown,
        }
    }

    /// The id of goods and armor, or of the weapon. For gems it's the source, the handle is `first_part`
    pub fn id(&self) -> u32 {
        match self.kind() {
            ItemKind::Good | ItemKind::Armor => self.second_part & ID_MASK,
            _ => self.second_part,
        }
    }

    pub(crate) fn parse(bytes: &[u8], at: usize) -> Self {
        Self {
            number: util::read_u32(bytes, at),
            first_part: util::read_u32(bytes, at + 4),
            second_part: util::read_u32(bytes, at + 8),
            amount: util::read_u32(bytes, at + 12),
        }
    }

    pub(crate) fn write(&self, bytes: &mut [u8], at: usize) {
        for (index, value) in [self.number, self.first_part, self.second_part, self.amount].into_iter().enumerate() {
            util::write_u32(bytes, at + index * 4, value);
        }
    }
}

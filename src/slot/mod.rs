use crate::util;

pub(crate) mod constants;
pub use constants::SLOT_COUNT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotShape {
    Radial,
    Triangle,
    Waning,
    Circle,
    Droplet,
    Closed,
    /// Values the game doesn't use, kept as they are
    Unknown(u32),
}

impl From<u32> for SlotShape {
    fn from(value: u32) -> Self {
        match value {
            0x01 => Self::Radial,
            0x02 => Self::Triangle,
            0x04 => Self::Waning,
            0x08 => Self::Circle,
            0x3F => Self::Droplet,
            0x8000_0000 => Self::Closed,
            other => Self::Unknown(other),
        }
    }
}

impl From<SlotShape> for u32 {
    fn from(shape: SlotShape) -> Self {
        match shape {
            SlotShape::Radial => 0x01,
            SlotShape::Triangle => 0x02,
            SlotShape::Waning => 0x04,
            SlotShape::Circle => 0x08,
            SlotShape::Droplet => 0x3F,
            SlotShape::Closed => 0x8000_0000,
            SlotShape::Unknown(other) => other,
        }
    }
}

/// A place for a gem in a weapon or armor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub shape: SlotShape,
    /// Handle of the gem in the slot, 0 if there is none
    pub gem: u32,
}

impl Slot {
    pub(crate) const LENGTH: usize = 8;

    pub(crate) fn parse(bytes: &[u8], at: usize) -> Self {
        Self { shape: util::read_u32(bytes, at).into(), gem: util::read_u32(bytes, at + 4) }
    }

    pub(crate) fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&u32::from(self.shape).to_le_bytes());
        out.extend_from_slice(&self.gem.to_le_bytes());
    }
}

use crate::util;

pub(crate) mod constants;
pub use constants::{EFFECT_COUNT, NO_EFFECT};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GemShape {
    Radial = 0x01,
    Triangle = 0x02,
    Waning = 0x04,
    Circle = 0x08,
    Droplet = 0x3F,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuneShape {
    Plain = 0x01,
    Oath = 0x02,
}

/// Blood gem or rune. The shapes available depend on which one it is
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GemKind {
    Gem(GemShape),
    Rune(RuneShape),
    /// Values the game doesn't use, kept as they are
    Unknown { kind: u32, shape: u32 },
}

impl GemKind {
    const GEM: u32 = 1;
    const RUNE: u32 = 2;

    fn from_raw(kind: u32, shape: u32) -> Self {
        match (kind, shape) {
            (Self::GEM, 0x01) => Self::Gem(GemShape::Radial),
            (Self::GEM, 0x02) => Self::Gem(GemShape::Triangle),
            (Self::GEM, 0x04) => Self::Gem(GemShape::Waning),
            (Self::GEM, 0x08) => Self::Gem(GemShape::Circle),
            (Self::GEM, 0x3F) => Self::Gem(GemShape::Droplet),
            (Self::RUNE, 0x01) => Self::Rune(RuneShape::Plain),
            (Self::RUNE, 0x02) => Self::Rune(RuneShape::Oath),
            _ => Self::Unknown { kind, shape },
        }
    }

    fn to_raw(self) -> (u32, u32) {
        match self {
            Self::Gem(shape) => (Self::GEM, shape as u32),
            Self::Rune(shape) => (Self::RUNE, shape as u32),
            Self::Unknown { kind, shape } => (kind, shape),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gem {
    /// What inventories and slots use to refer to the gem
    pub handle: u32,
    pub source: u32,
    pub kind: GemKind,
    pub effects: [u32; EFFECT_COUNT],
}

impl Gem {
    pub(crate) const LENGTH: usize = 40;

    pub(crate) fn parse(bytes: &[u8], at: usize) -> Self {
        let mut effects = [NO_EFFECT; EFFECT_COUNT];

        for (index, effect) in effects.iter_mut().enumerate() {
            *effect = util::read_u32(bytes, at + 16 + index * 4);
        }

        Self {
            handle: util::read_u32(bytes, at),
            source: util::read_u32(bytes, at + 4),
            kind: GemKind::from_raw(util::read_u32(bytes, at + 8), util::read_u32(bytes, at + 12)),
            effects,
        }
    }

    pub(crate) fn write(&self, out: &mut Vec<u8>) {
        let (kind, shape) = self.kind.to_raw();

        for value in [self.handle, self.source, kind, shape].into_iter().chain(self.effects) {
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
}

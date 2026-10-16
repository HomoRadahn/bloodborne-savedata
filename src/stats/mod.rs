use crate::{offsets::Offsets, util};

pub(crate) mod constants;
use constants::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stats {
    /// The player's health. Stored three times in a row, the copies can differ
    pub health: [u32; HEALTH_STAMINA_TIMES],
    /// The player's stamina. Stored three times in a row, the copies can differ
    pub stamina: [u32; HEALTH_STAMINA_TIMES],
    /// The amount of Blood Echoes
    pub blood_echoes: u32,
    /// The amount of Insight
    pub insight: u32,
    /// Level
    pub level: u32,
    /// Vitality stat value
    pub vitality: u32,
    /// Endurance stat value
    pub endurance: u32,
    /// Strength stat value
    pub strength: u32,
    /// Skill stat value
    pub skill: u32,
    /// Bloodtinge stat value
    pub bloodtinge: u32,
    /// Arcane stat value
    pub arcane: u32,
}

impl Stats {
    pub fn build(bytes: &[u8], offsets: &Offsets) -> Self {
        let health = std::array::from_fn(|index| util::get_relative_u32(bytes, offsets, HEALTH_OFFSET + (index * STAT_LENGTH) as isize, STAT_LENGTH));
        let stamina = std::array::from_fn(|index| util::get_relative_u32(bytes, offsets, STAMINA_OFFSET + (index * STAT_LENGTH) as isize, STAT_LENGTH));
        let blood_echoes = util::get_relative_u32(bytes, offsets, BLOOD_ECHOES_OFFSET, STAT_LENGTH);
        let insight = util::get_relative_u32(bytes, offsets, INSIGHT_OFFSET, STAT_LENGTH);
        let level = util::get_relative_u32(bytes, offsets, LEVEL_OFFSET, STAT_LENGTH);
        let vitality = util::get_relative_u32(bytes, offsets, VIT_OFFSET, STAT_LENGTH);
        let endurance = util::get_relative_u32(bytes, offsets, END_OFFSET, STAT_LENGTH);
        let strength = util::get_relative_u32(bytes, offsets, STR_OFFSET, STAT_LENGTH);
        let skill = util::get_relative_u32(bytes, offsets, SKL_OFFSET, STAT_LENGTH);
        let bloodtinge = util::get_relative_u32(bytes, offsets, BLT_OFFSET, STAT_LENGTH);
        let arcane = util::get_relative_u32(bytes, offsets, ARC_OFFSET, STAT_LENGTH);

        Self { health, stamina, blood_echoes, insight, level, vitality, endurance, strength, skill, bloodtinge, arcane }
    }

    /// Writes every stat back
    pub fn write(&self, bytes: &mut [u8], offsets: &Offsets) {
        let singles = [
            (BLOOD_ECHOES_OFFSET, self.blood_echoes),
            (INSIGHT_OFFSET, self.insight),
            (LEVEL_OFFSET, self.level),
            (VIT_OFFSET, self.vitality),
            (END_OFFSET, self.endurance),
            (STR_OFFSET, self.strength),
            (SKL_OFFSET, self.skill),
            (BLT_OFFSET, self.bloodtinge),
            (ARC_OFFSET, self.arcane),
        ];

        for (offset, value) in singles {
            util::set_relative_u32(bytes, offsets, offset, STAT_LENGTH, value);
        }

        for (start, values) in [(HEALTH_OFFSET, self.health), (STAMINA_OFFSET, self.stamina)] {
            for (index, value) in values.into_iter().enumerate() {
                util::set_relative_u32(bytes, offsets, start + (index * STAT_LENGTH) as isize, STAT_LENGTH, value);
            }
        }
    }
}

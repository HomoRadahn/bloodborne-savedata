use crate::{item::{Item, ItemKind}, util};

/// Every record of a container, empty ones included. Moving or adding things means changing the records
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inventory {
    pub items: Vec<Item>,
    /// How many records the game believes are in use, if it keeps track of that. The first one of the storage is
    /// stored in the same place as the `number` of its first record, the latter wins when writing
    pub counters: Option<(u32, u32)>,
}

impl Inventory {
    pub(crate) fn parse(bytes: &[u8], range: (usize, usize), counters: Option<(usize, usize)>) -> Self {
        Self {
            items: (range.0..range.1.saturating_sub(Item::LENGTH - 1))
                .step_by(Item::LENGTH)
                .map(|at| Item::parse(bytes, at))
                .collect(),
            counters: counters.map(|(first, second)| (util::read_u32(bytes, first), util::read_u32(bytes, second))),
        }
    }

    pub(crate) fn write(&self, bytes: &mut [u8], range: (usize, usize), counters: Option<(usize, usize)>) {
        if let (Some((first, second)), Some((first_value, second_value))) = (counters, self.counters) {
            util::write_u32(bytes, first, first_value);
            util::write_u32(bytes, second, second_value);
        }

        for (index, item) in self.items.iter().enumerate().take_while(|(index, _)| range.0 + (index + 1) * Item::LENGTH <= range.1) {
            item.write(bytes, range.0 + index * Item::LENGTH);
        }
    }

    /// Records that hold something
    pub fn occupied(&self) -> impl Iterator<Item = &Item> {
        self.items.iter().filter(|item| item.kind() != ItemKind::Empty)
    }
}

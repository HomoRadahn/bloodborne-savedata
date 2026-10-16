use std::io::{self, ErrorKind::InvalidData};

use crate::offsets::Offsets;

pub(crate) mod constants;
use constants::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    /// Map in which the player is located
    pub map: (u8, u8, u8, u8),
    /// Player's last saved x position
    pub x: f32,
    /// Player's last saved y position
    pub y: f32,
    /// Player's last saved z position
    pub z: f32,
    /// Offset where the position was found
    pub offset: usize
}

impl Position {
    pub fn build(bytes: &[u8], offsets: &Offsets) -> io::Result<Position> {
        for index in offsets.lced..(bytes.len() - MARKER.len()) {
            if MARKER == bytes[index..index + MARKER.len()] {
                let at = index + COORDINATES_OFFSET;
                let read = |at: usize| f32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);

                return Ok(Self {
                    map: (bytes[MAP_OFFSET + 3], bytes[MAP_OFFSET + 2], bytes[MAP_OFFSET + 1], bytes[MAP_OFFSET]),
                    x: read(at),
                    y: read(at + 4),
                    z: read(at + 8),
                    offset: index,
                });
            }
        }

        Err(io::Error::new(InvalidData, "Coordinates could not be found"))
    }

    /// Writes the coordinates and the loaded map to the savedata
    pub fn write(&self, bytes: &mut [u8]) {
        for (index, value) in [self.x, self.y, self.z].into_iter().enumerate() {
            let at = self.offset + COORDINATES_OFFSET + index * 4;

            bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
        }

        bytes[MAP_OFFSET..MAP_OFFSET + 4].copy_from_slice(&[self.map.3, self.map.2, self.map.1, self.map.0]);
    }
}

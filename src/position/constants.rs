/// The coordinates are preceded by this sequence, somewhere after the LCED marker
pub const MARKER: [u8; 12] = [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

/// Distance from the start of the marker to the x coordinate. y and z follow, `f32` each
pub const COORDINATES_OFFSET: usize = 12;

/// The loaded map is stored at the start of the file, `m21_00_00_00` becoming `00 00 00 15`
pub const MAP_OFFSET: usize = 4;

use crate::offsets::Offsets;

pub(crate) fn read_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

pub(crate) fn write_u32(bytes: &mut [u8], at: usize, value: u32) {
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

/// Resolves an offset relative to the character name into an absolute one
pub(crate) fn absolute(offsets: &Offsets, relative: isize) -> usize {
    (offsets.character_name as isize + relative) as usize
}

pub(crate) fn get_relative_u32(bytes: &[u8], offsets: &Offsets, offset: isize, length: usize) -> u32 {
    let absolute_offset = absolute(offsets, offset);
    let bytes = &bytes[absolute_offset..absolute_offset + length];

    let mut out = 0;

    for (index, byte) in bytes.iter().enumerate().rev() {
        out += *byte as u32 * (256_u32.pow(index as u32));
    }

    out
}

/// Writes the low `length` bytes of `value` at the relative offset
pub(crate) fn set_relative_u32(bytes: &mut [u8], offsets: &Offsets, offset: isize, length: usize, value: u32) {
    let start = absolute(offsets, offset);

    bytes[start..start + length].copy_from_slice(&value.to_le_bytes()[..length]);
}

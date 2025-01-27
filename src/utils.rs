/// Joins two u8 values into an u16.
pub fn join_8_to_16(x: u8, y: u8) -> u16 {
    (x as u16) << 8 | y as u16
}

/// Splits an u16 into two u8 values.
pub fn split_16_to_8(x: u16) -> (u8, u8) {
    (((x & 0xff00) >> 8) as u8, (x & 0x00ff) as u8)
}

/// Joins two u8 values into an u16, little endian order.
pub fn join_8_to_16_lsf(x: u8, y: u8) -> u16 {
    (y as u16) << 8 | x as u16
}

/// Splits an u16 into two u8 values, little endian order.
pub fn split_16_to_8_lsf(x: u16) -> (u8, u8) {
    ((x & 0x00ff) as u8, ((x & 0xff00) >> 8) as u8)
}

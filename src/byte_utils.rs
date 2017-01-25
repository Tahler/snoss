pub fn u32_from_bytes(bytes: [u8; 4]) -> u32 {
    let byte_0 = (bytes[0] as u32) << 24;
    let byte_1 = (bytes[1] as u32) << 16;
    let byte_2 = (bytes[2] as u32) << 08;
    let byte_3 = (bytes[3] as u32) << 00;
    byte_0 + byte_1 + byte_2 + byte_3
}

pub fn u32_to_bytes(val: u32) -> [u8; 4] {
    let byte_0 = ((0xFF000000 & val) >> 24) as u8;
    let byte_1 = ((0x00FF0000 & val) >> 16) as u8;
    let byte_2 = ((0x0000FF00 & val) >> 08) as u8;
    let byte_3 = ((0x000000FF & val) >> 00) as u8;
    [byte_0, byte_1, byte_2, byte_3]
}

pub fn u16_from_bytes(bytes: [u8; 2]) -> u16 {
    let high_byte = bytes[0] as u16;
    let low_byte = bytes[1] as u16;
    let high_byte_shifted = high_byte << 8;
    high_byte_shifted + low_byte
}

pub fn u16_to_bytes(val: u16) -> [u8; 2] {
    let high_byte = (val >> 8) as u8; // TODO: should not roll
    let low_byte = val as u8; // TODO: should truncate
    [high_byte, low_byte]
}

pub fn set_u16_at(bytes: &mut [u8], addr: usize, val: u16) {
    let u16_bytes = u16_to_bytes(val);
    bytes[addr..addr + 1].clone_from_slice(&u16_bytes);
}

pub fn get_u16_at(bytes: &[u8], addr: usize) -> u16 {
    let u16_bytes = &bytes[addr..addr + 1];
    u16_from_bytes([u16_bytes[0], u16_bytes[1]])
}

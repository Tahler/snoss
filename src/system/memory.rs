use byte_utils;

const STACK_PTR_LOC: usize = 0;
const USABLE_MEM_START: usize = 4;

#[derive(Debug)]
pub struct Memory {
    pub bytes: Box<[u8]>, // Store the stack_ptr at bytes 0-1?
}

impl Memory {
    pub fn new(num_bytes: usize) -> Self {
        let mut ram = Memory { bytes: vec![0; num_bytes].into_boxed_slice() };
        ram.set_stack_ptr(USABLE_MEM_START as u16);
        ram
    }

    pub fn get_u8(&self, addr: usize) -> u8 {
        self.bytes[addr]
    }

    pub fn set_u8(&mut self, addr: usize, val: u8) {
        self.bytes[addr] = val;
    }

    pub fn get_u16(&self, addr: usize) -> u16 {
        let bytes = [self.bytes[addr], self.bytes[addr + 1]];
        byte_utils::u16_from_bytes(bytes)
    }

    pub fn set_u16(&mut self, addr: usize, val: u16) {
        let bytes = byte_utils::u16_to_bytes(val);
        let high_byte = bytes[0];
        self.set_u8(addr, high_byte);
        let low_byte = bytes[1];
        self.set_u8(addr + 1, low_byte);
    }

    pub fn get_stack_ptr(&self) -> u16 {
        self.get_u16(STACK_PTR_LOC)
    }

    pub fn set_stack_ptr(&mut self, val: u16) {
        self.set_u16(STACK_PTR_LOC, val);
    }
}

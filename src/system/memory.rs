use byte_utils;

use super::instruction::Instructions;

const STACK_PTR_LOC: usize = 0;
const USABLE_MEM_START: usize = 4;

#[derive(Debug)]
pub struct Memory {
    bytes: Box<[u8]>, // Store the stack_ptr at bytes 0-1?
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

    fn get_stack_ptr(&self) -> u16 {
        self.get_u16(STACK_PTR_LOC)
    }

    fn set_stack_ptr(&mut self, val: u16) {
        self.set_u16(STACK_PTR_LOC, val);
    }

    /// Returns the address of the first byte of the allocated block.
    pub fn allocate(&mut self, num_bytes: usize) -> Result<Block, String> {
        let stack_ptr = self.get_stack_ptr();
        let new_stack_ptr = stack_ptr + (num_bytes as u16);
        let total_bytes = self.bytes.len() as u16;
        if new_stack_ptr <= total_bytes {
            self.set_stack_ptr(new_stack_ptr);
            let block_start = stack_ptr as usize;
            let block_end = new_stack_ptr as usize;
            let allocated_bytes = &mut self.bytes[block_start..block_end];
            Ok(Block::new(allocated_bytes))
        } else {
            Err(format!("Not enough memory to allocate {} bytes.", num_bytes))
        }
    }

    // TODO: deallocate - maybe include size of block right before?
}

#[derive(Debug)]
pub struct Block<'a> {
    mem_slice: &'a mut [u8],
}

impl<'a> Block<'a> {
    pub fn new(mem_slice: &'a mut [u8]) -> Block<'a> {
        Block { mem_slice: mem_slice }
    }

    pub fn get_u8(&self, addr: usize) -> u8 {
        self.mem_slice[addr]
    }

    pub fn set_u8(&mut self, addr: usize, val: u8) {
        self.mem_slice[addr] = val;
    }

    pub fn get_u16(&self, addr: usize) -> u16 {
        let bytes = [self.mem_slice[addr], self.mem_slice[addr + 1]];
        byte_utils::u16_from_bytes(bytes)
    }

    pub fn set_u16(&mut self, addr: usize, val: u16) {
        let bytes = byte_utils::u16_to_bytes(val);
        let high_byte = bytes[0];
        self.set_u8(addr, high_byte);
        let low_byte = bytes[1];
        self.set_u8(addr + 1, low_byte);
    }

    pub fn len(&self) -> usize {
        self.mem_slice.len()
    }
}

#[derive(Debug)]
pub struct ProcessControlBlock<'a> {
    block: Block<'a>,
}

impl<'a> ProcessControlBlock<'a> {
    pub fn new(block: Block<'a>) -> ProcessControlBlock<'a> {
        ProcessControlBlock { block: block }
    }

    // pub fn get_process_id(&self) -> u16 {
    //     byte_utils::u16_from_bytes(self.mem_slice[0..2])
    // }

    // pub fn get_stack_size(&self) -> u16 {
    //     byte_utils::u16_from_bytes(self.mem_slice[2..4])
    // }
}

#[derive(Debug)]
pub struct InstructionBlock<'a> {
    block: Block<'a>,
}

impl<'a> InstructionBlock<'a> {
    pub fn new(block: Block<'a>) -> Result<InstructionBlock<'a>, String> {
        let instruction_size = super::instruction::INSTRUCTION_SIZE;
        if block.len() % instruction_size == 0 {
            let block = InstructionBlock { block: block };
            Ok(block)
        } else {
            Err(format!("An instruction block's size must be a multiple of the instruction size \
                         ({} bytes).",
                        instruction_size))
        }
    }

    pub fn get_instructions(&self) -> Instructions {
        let bytes = &self.block.mem_slice;
        Instructions::new(bytes)
    }
}

use byte_utils;

use super::instruction::{Instruction, INSTRUCTION_SIZE};

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

#[derive(Debug)]
pub struct InstructionBlock<'a> {
    mem_slice: &'a [u8],
}

impl<'a> InstructionBlock<'a> {
    pub fn new(mem_slice: &'a [u8]) -> Result<InstructionBlock<'a>, String> {
        if mem_slice.len() % INSTRUCTION_SIZE == 0 {
            let block = InstructionBlock { mem_slice: mem_slice };
            Ok(block)
        } else {
            Err(format!("An instruction block's size must be a multiple of the instruction size \
                         ({} bytes).",
                        INSTRUCTION_SIZE))
        }
    }

    pub fn get_instruction_at(&self, addr: usize) -> Instruction {
        let bytes = &self.mem_slice[addr..addr + INSTRUCTION_SIZE];
        Instruction::from_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

// #[derive(Debug)]
// pub struct ProcessControlBlock<'a> {
//     block: Block<'a>,
// }

// const METADATA_SIZE: usize = 4;
// const PROCESS_ID_LOC: usize = 0;
// const STACK_SIZE_LOC: usize = 2;

// impl<'a> ProcessControlBlock<'a> {
//     pub fn new(mut block: Block<'a>, process_id: u16) -> ProcessControlBlock<'a> {
//         block.set_u16(PROCESS_ID_LOC, process_id);

//         let stack_size = (block.len() - METADATA_SIZE) as u16;
//         block.set_u16(STACK_SIZE_LOC, stack_size);

//         ProcessControlBlock { block: block }
//     }

//     pub fn get_process_id(&self) -> u16 {
//         let pid_bytes = [self.block.mem_slice[PROCESS_ID_LOC],
//                          self.block.mem_slice[PROCESS_ID_LOC + 1]];
//         byte_utils::u16_from_bytes(pid_bytes)
//     }

//     pub fn get_stack_size(&self) -> u16 {
//         let stack_size_bytes = [self.block.mem_slice[STACK_SIZE_LOC],
//                                 self.block.mem_slice[STACK_SIZE_LOC + 1]];
//         byte_utils::u16_from_bytes(stack_size_bytes)
//     }
// }

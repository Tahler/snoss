use std::fmt;

use byte_utils;

use super::cpu::WORD_LEN;

const PROC_ID_LOC: usize = 0;
const PROC_STATUS_LOC: usize = PROC_ID_LOC + WORD_LEN;
const INSTR_PTR_LOC: usize = PROC_STATUS_LOC + WORD_LEN;
const INSTR_BLOCK_ADDR_LOC: usize = INSTR_PTR_LOC + WORD_LEN;
const STACK_LEN_LOC: usize = INSTR_BLOCK_ADDR_LOC + WORD_LEN;
pub const STACK_LOC: usize = STACK_LEN_LOC + WORD_LEN;

pub const PCB_METADATA_LEN: usize = STACK_LOC;
pub const STACK_LEN: usize = 64;
pub const PCB_LEN: usize = PCB_METADATA_LEN + STACK_LEN;

pub struct ProcessControlBlock<'a> {
    bytes: &'a mut [u8],
}

impl<'a> ProcessControlBlock<'a> {
    pub fn new(bytes: &'a mut [u8],
               proc_id: u16,
               instr_blk_addr: u16)
               -> Result<ProcessControlBlock<'a>, String> {
        if bytes.len() == PCB_LEN {
            let stack_len = (bytes.len() - PCB_METADATA_LEN) as u16;

            let mut block = ProcessControlBlock { bytes: bytes };
            block.set_proc_id(proc_id);
            block.set_proc_status(ProcessStatus::None);
            block.set_instr_ptr(0);
            block.set_instr_blk_addr(instr_blk_addr);
            block.set_stack_len(stack_len);

            Ok(block)
        } else {
            Err(format!("Mem slice is not of the correct size. expected: {:?} actual: {:?}",
                        PCB_LEN,
                        bytes.len()))
        }
    }

    pub fn get_stack(&self) -> &[u8] {
        &self.bytes[PCB_METADATA_LEN..]
    }

    pub fn get_stack_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[PCB_METADATA_LEN..]
    }

    pub fn get_proc_id(&self) -> u16 {
        byte_utils::get_u16_at(self.bytes, PROC_ID_LOC).unwrap()
    }

    pub fn set_proc_id(&mut self, proc_id: u16) {
        byte_utils::set_u16_at(self.bytes, PROC_ID_LOC, proc_id).unwrap()
    }

    pub fn get_proc_status(&self) -> ProcessStatus {
        use enum_primitive::FromPrimitive;
        let proc_status_num = byte_utils::get_u16_at(self.bytes, PROC_STATUS_LOC).unwrap();
        ProcessStatus::from_u16(proc_status_num).unwrap()
    }

    pub fn set_proc_status(&mut self, proc_status: ProcessStatus) {
        let proc_status_num = proc_status as u16;
        byte_utils::set_u16_at(self.bytes, PROC_STATUS_LOC, proc_status_num);
    }

    pub fn get_instr_ptr(&self) -> u16 {
        byte_utils::get_u16_at(self.bytes, INSTR_PTR_LOC).unwrap()
    }

    pub fn set_instr_ptr(&mut self, instr_ptr: u16) {
        byte_utils::set_u16_at(self.bytes, INSTR_PTR_LOC, instr_ptr);
    }

    pub fn get_instr_blk_addr(&self) -> u16 {
        byte_utils::get_u16_at(self.bytes, INSTR_BLOCK_ADDR_LOC).unwrap()
    }

    fn set_instr_blk_addr(&mut self, instr_blk_addr: u16) {
        byte_utils::set_u16_at(self.bytes, INSTR_BLOCK_ADDR_LOC, instr_blk_addr);
    }

    pub fn get_stack_len(&self) -> u16 {
        let stack_len_bytes = [self.bytes[STACK_LEN_LOC], self.bytes[STACK_LEN_LOC + 1]];
        byte_utils::u16_from_bytes(stack_len_bytes)
    }

    fn set_stack_len(&mut self, stack_len: u16) {
        byte_utils::set_u16_at(self.bytes, STACK_LEN_LOC, stack_len);
    }
}

impl<'a> fmt::Debug for ProcessControlBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ProcessControlBlock: {{ proc_id: {:?}, proc_status: {:?}, stack: {:?} }}",
               self.get_proc_id(),
               self.get_proc_status(),
               self.get_stack())
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum ProcessStatus {
    None = 0
}
}

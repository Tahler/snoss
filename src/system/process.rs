use byte_utils;

const PCB_METADATA_LEN: usize = 2 + 2 + 2 + 2 + 2;
const STACK_LEN: usize = 1024;
pub const PCB_LEN: usize = PCB_METADATA_LEN + STACK_LEN;

const PROC_ID_LOC: usize = 0;
const PROC_STATUS_LOC: usize = 2;
const INSTR_PTR_LOC: usize = 4;
const INSTR_BLOCK_ADDR_LOC: usize = 6;
const STACK_LEN_LOC: usize = 8;

#[derive(Debug)]
pub struct ProcessControlBlock<'a> {
    mem_slice: &'a mut [u8],
}

impl<'a> ProcessControlBlock<'a> {
    pub fn new(mem_slice: &'a mut [u8],
               proc_id: u16,
               instr_blk_addr: u16)
               -> Result<ProcessControlBlock<'a>, String> {
        if mem_slice.len() == PCB_LEN {
            let stack_len = (mem_slice.len() - PCB_METADATA_LEN) as u16;

            let mut block = ProcessControlBlock { mem_slice: mem_slice };
            block.set_proc_id(proc_id);
            block.set_proc_status(ProcessStatus::None);
            block.set_instr_ptr(0);
            block.set_instr_blk_addr(instr_blk_addr);
            block.set_stack_len(stack_len);

            Ok(block)
        } else {
            Err("Mem slice is not of the correct size.".to_string())
        }
    }

    pub fn get_proc_id(&self) -> u16 {
        byte_utils::get_u16_at(self.mem_slice, PROC_ID_LOC)
    }

    pub fn set_proc_id(&mut self, proc_id: u16) {
        byte_utils::set_u16_at(self.mem_slice, PROC_ID_LOC, proc_id)
    }

    pub fn get_proc_status(&self) -> ProcessStatus {
        use enum_primitive::FromPrimitive;
        let proc_status_num = byte_utils::get_u16_at(self.mem_slice, PROC_STATUS_LOC);
        ProcessStatus::from_u16(proc_status_num).unwrap()
    }

    pub fn set_proc_status(&mut self, proc_status: ProcessStatus) {
        let proc_status_num = proc_status as u16;
        byte_utils::set_u16_at(self.mem_slice, PROC_STATUS_LOC, proc_status_num);
    }

    pub fn get_instr_ptr(&self) -> u16 {
        byte_utils::get_u16_at(self.mem_slice, INSTR_PTR_LOC)
    }

    pub fn set_instr_ptr(&mut self, instr_ptr: u16) {
        byte_utils::set_u16_at(self.mem_slice, INSTR_PTR_LOC, instr_ptr);
    }

    pub fn get_instr_blk_addr(&self) -> u16 {
        byte_utils::get_u16_at(self.mem_slice, INSTR_BLOCK_ADDR_LOC)
    }

    fn set_instr_blk_addr(&mut self, instr_blk_addr: u16) {
        byte_utils::set_u16_at(self.mem_slice, INSTR_BLOCK_ADDR_LOC, instr_blk_addr);
    }

    pub fn get_stack_len(&self) -> u16 {
        let stack_len_bytes = [self.mem_slice[STACK_LEN_LOC], self.mem_slice[STACK_LEN_LOC + 1]];
        byte_utils::u16_from_bytes(stack_len_bytes)
    }

    fn set_stack_len(&mut self, stack_len: u16) {
        byte_utils::set_u16_at(self.mem_slice, STACK_LEN_LOC, stack_len);
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum ProcessStatus {
    None = 0
}
}

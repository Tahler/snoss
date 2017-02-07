use std::fmt;
use super::cpu::Cpu;
use super::instr::{InstructionBlock, INSTRUCTION_LEN};

pub const PCB_LEN: usize = HEADER_LEN + STACK_LEN + INSTRUCTION_BLK_LEN;
const HEADER_LEN: usize = 2 + 2 + CTX_LEN;
const CTX_LEN: usize = 14;
// 64 bytes stack limit
pub const STACK_LEN: usize = 64;
// 256 instructions limit
pub const INSTRUCTION_BLK_LEN: usize = 256 * INSTRUCTION_LEN;

#[derive(Debug)]
pub struct Pcb {
    header: PcbHeader,
    pub stack: Stack,
    pub instr: InstructionBlock,
}

#[derive(Debug)]
struct PcbHeader {
    pub id: u16,
    pub status: ProcStatus,
    pub ctx: ProcContext,
}

#[derive(Debug, PartialEq)]
pub enum ProcStatus {
    None = 0,
}

#[derive(Debug)]
struct ProcContext {
    instr_ptr: u16,
    reg_1: u16,
    reg_2: u16,
    reg_3: u16,
    reg_4: u16,
    reg_5: u16,
    reg_6: u16,
}

pub struct Stack {
    pub bytes: [u8; STACK_LEN],
}

impl Pcb {
    pub fn new(proc_id: u16, instr: InstructionBlock) -> Pcb {
        Pcb {
            header: PcbHeader {
                id: proc_id,
                status: ProcStatus::None,
                ctx: ProcContext::new(),
            },
            stack: Stack::new(),
            instr: instr,
        }
    }

    pub fn save_ctx(&mut self, cpu: &Cpu) {
        let mut ctx = &mut self.header.ctx;
        ctx.instr_ptr = cpu.instr_ptr as u16;
        ctx.reg_1 = cpu.registers[0];
        ctx.reg_2 = cpu.registers[1];
        ctx.reg_3 = cpu.registers[2];
        ctx.reg_4 = cpu.registers[3];
        ctx.reg_5 = cpu.registers[4];
        ctx.reg_6 = cpu.registers[5];
    }

    // TODO: this should maybe be in os.rs
    // pub fn load_ctx() {}

    pub fn get_stack(&self) -> &[u8] {
        &self.stack.bytes
    }

    pub fn get_stack_mut(&mut self) -> &mut [u8] {
        &mut self.stack.bytes
    }

    pub fn get_instr_blk(&self) -> &InstructionBlock {
        &self.instr
    }

    pub fn get_id(&self) -> u16 {
        self.header.id
    }

    pub fn set_id(&mut self, proc_id: u16) {
        self.header.id = proc_id;
    }

    pub fn get_status(&self) -> &ProcStatus {
        &self.header.status
    }

    pub fn set_status(&mut self, proc_status: ProcStatus) {
        self.header.status = proc_status;
    }

    pub fn get_instr_ptr(&self) -> u16 {
        self.header.ctx.instr_ptr
    }

    pub fn set_instr_ptr(&mut self, instr_ptr: u16) {
        self.header.ctx.instr_ptr = instr_ptr;
    }
}

impl Stack {
    fn new() -> Self {
        Stack { bytes: [0; STACK_LEN] }
    }
}

impl ProcContext {
    fn new() -> Self {
        ProcContext {
            instr_ptr: 0,
            reg_1: 0,
            reg_2: 0,
            reg_3: 0,
            reg_4: 0,
            reg_5: 0,
            reg_6: 0,
        }
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack {:?}", &self.bytes[..])
    }
}

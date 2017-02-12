use std::fmt;
use os::consts::STACK_LEN;
use super::super::cpu::Cpu;
use super::super::instr::{InstructionBlock, INSTRUCTION_LEN};

// Effectively `std::mem::size_of::<Pcb>()`
pub const PCB_LEN: usize = HEADER_LEN + CTX_LEN + INSTRUCTION_BLK_LEN;
const HEADER_LEN: usize = 2 + 2 + CTX_LEN;
const CTX_LEN: usize = 14;
// 256 instructions limit
pub const INSTRUCTION_BLK_LEN: usize = 256 * INSTRUCTION_LEN;

#[derive(Debug)]
pub struct Pcb {
    header: Header,
    pub stack: Stack,
    pub instr: InstructionBlock,
}

// TODO: is this automatically impl?
// unsafe impl Send for Pcb {
// }

#[derive(Debug)]
struct Header {
    pub id: u16,
    pub status: Status,
    pub ctx: Context,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    None = 0,
}

type Context = Cpu;

pub struct Stack {
    pub bytes: [u8; STACK_LEN],
}

impl Pcb {
    pub fn new(proc_id: u16, instr: InstructionBlock) -> Pcb {
        Pcb {
            header: Header {
                id: proc_id,
                status: Status::None,
                ctx: Context::new(),
            },
            stack: Stack::new(),
            instr: instr,
        }
    }

    pub fn load_cpu_ctx(&self, cpu: &mut Cpu) {
        let ctx = &self.header.ctx;
        cpu.instr_ptr = ctx.instr_ptr;
        cpu.registers.clone_from_slice(&ctx.registers);
    }

    pub fn save_cpu_ctx(&mut self, cpu: &Cpu) {
        let mut ctx = &mut self.header.ctx;
        ctx.instr_ptr = cpu.instr_ptr;
        ctx.registers.clone_from_slice(&cpu.registers[..]);
    }

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

    pub fn get_status(&self) -> &Status {
        &self.header.status
    }

    pub fn set_status(&mut self, proc_status: Status) {
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

impl Context {
    fn new() -> Self {
        use os::consts::NUM_REGISTERS;
        Cpu {
            instr_ptr: 0,
            registers: [0; NUM_REGISTERS],
        }
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack {:?}", &self.bytes[..])
    }
}

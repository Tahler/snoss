use std::fmt;

pub const WORD_LEN: usize = 2;

pub struct Cpu {
    pub instr_ptr: usize,
    pub registers: Box<[u16]>,
}

impl Cpu {
    pub fn new(num_registers: usize) -> Self {
        Cpu {
            instr_ptr: 0,
            registers: vec![0; num_registers].into_boxed_slice(),
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ IP: 0x{:x}, REG_FILE: {:?} }}", self.instr_ptr, self.registers)
    }
}

use std::fmt;
use byte_utils::AccessResult;

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

    pub fn get_reg(&self, addr: usize) -> AccessResult<u16> {
        self.registers.get(addr).map(|val| *val).ok_or(())
    }

    pub fn set_reg(&mut self, addr: usize, val: u16) -> AccessResult<()> {
        self.registers.get_mut(addr).map(|curr_val| *curr_val = val).ok_or(())
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Cpu: {{ instr_ptr: 0x{:x}, registers: {:?} }}",
               self.instr_ptr,
               self.registers)
    }
}

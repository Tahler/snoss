use std::fmt;
use byte_utils::AccessResult;
use os::consts::NUM_REGISTERS;

pub struct Cpu {
    pub instr_ptr: u16,
    pub registers: [u16; NUM_REGISTERS],
}

impl Cpu {
    pub fn init() -> Self {
        Cpu {
            instr_ptr: 0,
            registers: [0; NUM_REGISTERS],
        }
    }

    pub fn get_reg(&self, addr: u8) -> AccessResult<u16> {
        let addr = addr as usize;
        self.registers.get(addr).map(|val| *val).ok_or(())
    }

    pub fn set_reg(&mut self, addr: u8, val: u16) -> AccessResult<()> {
        let addr = addr as usize;
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

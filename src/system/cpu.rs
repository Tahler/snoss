#[derive(Debug)]
pub struct Cpu {
    pub instruction_ptr: usize,
    pub registers: Box<[u16]>,
}

impl Cpu {
    pub fn new(num_registers: usize) -> Self {
        Cpu {
            instruction_ptr: 0,
            registers: vec![0; num_registers].into_boxed_slice(),
        }
    }
}

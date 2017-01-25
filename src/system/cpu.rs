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

    // pub fn to_string(&self) -> String {
    //     let reg_slice = &self.registers;
    //     format!("{:?}", reg_slice)
    // }
}

use super::cpu::Cpu;
use super::memory::Memory;
use super::storage::FileSystem;

#[derive(Debug)]
pub struct System {
    cpu: Cpu,
    ram: Memory,
    fs: FileSystem,
}

impl System {
    pub fn new() -> Self {
        System {
            cpu: Cpu::new(6),
            ram: Memory::new(10_000),
            fs: FileSystem::new("./fs"),
        }
    }

    pub fn list_files(&self) -> String {
        self.fs.list_files()
    }

    // pub fn exec(&mut self, file: &str) {
    //     self.load_program(file);
    // }

    // /// Loads the specified file into memory.
    // // TODO: perhaps returns the memory address?
    // // TODO: perhaps sets the instruction_ptr?
    // fn load_program(&mut self, file: &str) {
    //     unimplemented!()
    // }

    // fn exec(&mut self) {
    //     loop {
    //         // Load instruction_ptr
    //         let instruction_ptr = self.cpu.instruction_ptr;
    //         // Increment instruction_ptr
    //         self.cpu.instruction_ptr += 1;
    //         // Load the instruction at
    //         let instruction = self.ram.load(instruction_ptr);
    //         // Execute instruction at instruction_ptr
    //         self.exec_instruction(instruction);
    //     }
    // }

    // fn exec_instruction(&mut self, instruction: u32) {
    //     let byte0 = instruction >> 28; // TODO:
    //     let byte1 = instruction >> 28:

    //     unimplemented!();
    // }

    // fn to_string(&self) -> String {
    //     let reg_slice: &[u32] = self.cpu.registers.as_ref();
    //     format!("{:?}", reg_slice)
    // }

    // fn load_constant(&mut self, constant: u32, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = constant;
    // }

    // fn add(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] + reg_slice[src_reg_b]
    // }

    // fn sub(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] - reg_slice[src_reg_b]
    // }

    // fn mul(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] * reg_slice[src_reg_b]
    // }

    // fn div(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] / reg_slice[src_reg_b]
    // }

    // fn eq(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     let dest_val = if reg_slice[src_reg_a] == reg_slice[src_reg_b] {
    //         0x01
    //     } else {
    //         0x00
    //     };
    //     reg_slice[dest_reg] = dest_val
    // }

    // fn goto(&mut self, addr: u32) {
    //     self.cpu.instruction_ptr = addr;
    // }

    // fn goto_if(&mut self, addr: usize, reg: usize) {
    //     let reg_slice = self.cpu.registers.as_ref();
    //     if reg_slice[reg] == 0 {
    //         self.cpu.instruction_ptr = addr;
    //     }
    // }
}

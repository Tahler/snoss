use super::cpu::Cpu;
use super::storage::FileSystem;
use super::memory::{Memory, InstructionBlock};
use super::instruction::Instruction;

const PCB_SIZE: usize = 1024;

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

    pub fn exec(&mut self, file: &str) -> Result<String, String> {
        // TODO: Only ram needs to be borrowed mutably for a short period

        let instr_block = self.load_program(file)?;
        // let mut ram = &mut self.ram;
        // let block = ram.allocate_block(1024)?;
        // let mut pcb = ProcessControlBlock::new(block, 1);
        // self.exec_program(&instr_block, &mut pcb);
        unimplemented!()
    }

    /// Loads the specified file into memory.
    fn load_program(&mut self, file: &str) -> Result<InstructionBlock, String> {
        let file_bytes: Vec<u8> = self.fs.open_bytes(file)?
            .map(|result| result.unwrap())
            .collect();
        let num_bytes = file_bytes.len();

        let stack_ptr = self.ram.get_stack_ptr();
        let start = stack_ptr as usize;
        let end = start + num_bytes;
        {
            self.ram.bytes[start..end].clone_from_slice(&file_bytes[..]);
        }
        {
            let new_stack_ptr = end as u16;
            self.ram.set_stack_ptr(new_stack_ptr);
        }
        let instr_block = InstructionBlock::new(&self.ram.bytes[start..end])?;
        Ok(instr_block)
    }

    // fn exec_program(&mut self, instruction_block: &InstructionBlock, pcb: &mut ProcessControlBlock) {
    //     loop {
    //         // Load instruction_ptr
    //         let instruction_ptr = self.cpu.instruction_ptr;
    //         // Increment instruction_ptr
    //         self.cpu.instruction_ptr += 1;
    //         let instruction = instruction_block.get_instruction_at(instruction_ptr);
    //         // Execute instruction at instruction_ptr
    //         self.exec_instruction(&instruction);
    //     }
    // }

    fn exec_instruction(&mut self, instruction: &Instruction) {
        unimplemented!();
    }

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

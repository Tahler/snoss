// RAM
// 0x0000: stack_ptr
// ...
// pcb: proc_id
// pcb: proc_status
// pcb: instr_blk_ptr
// pcb: instr_ptr
// pcb: stack_len
// pcb: [stack...]
// ...
// instr: [instr...]
// ...

use super::super::byte_utils;

use super::cpu::Cpu;
use super::storage::FileSystem;
use super::memory::Memory;
use super::instruction::{Instruction, InstructionBlock};
use super::process::{PCB_LEN, ProcessControlBlock};

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
        // Determine alloc sizes
        // /////////////////////////////////////////////////////////////////////////////////////////
        let file_bytes = self.fs.open_bytes_as_vec(file)?;
        let num_instr_bytes = file_bytes.len();

        let stack_ptr = self.ram.get_stack_ptr();

        let instr_blk_start = stack_ptr as usize;
        let instr_blk_end = instr_blk_start + num_instr_bytes;

        let proc_contr_blk_start = instr_blk_end;
        let proc_contr_blk_end = proc_contr_blk_start + PCB_LEN;

        let new_stack_ptr = proc_contr_blk_end as u16;
        self.ram.set_stack_ptr(new_stack_ptr);

        let mut available_mem = &mut self.ram.bytes[instr_blk_start..];

        // Create instr_block
        // /////////////////////////////////////////////////////////////////////////////////////////
        // TODO: maybe split into functions through the split
        let (mut instr_blk_alloc, mut available_mem) = available_mem.split_at_mut(instr_blk_start);
        instr_blk_alloc[instr_blk_start..instr_blk_end].clone_from_slice(&file_bytes[..]);
        let instr_block = InstructionBlock::new(&instr_blk_alloc[instr_blk_start..instr_blk_end])?;

        // Create proc_contr_blk
        // /////////////////////////////////////////////////////////////////////////////////////////
        let (mut proc_contr_blk_alloc, mut available_mem) =
            available_mem.split_at_mut(proc_contr_blk_start);

        let mut pcb = ProcessControlBlock::new(proc_contr_blk_alloc, 1, instr_blk_start as u16)?;
        // self.exec_program(&instr_block, &mut pcb);

        unimplemented!()
    }

    /// Loads the specified file into memory.
    // fn load_program(&mut self, file: &str) -> Result<InstructionBlock, String> {
    //     let file_bytes = self.fs.open_bytes_as_vec(file)?;
    //     let num_bytes = file_bytes.len();

    //     let stack_ptr = self.ram.get_stack_ptr();
    //     let start = stack_ptr as usize;
    //     let end = start + num_bytes;

    //     let new_stack_ptr = end as u16;
    //     self.ram.set_stack_ptr(new_stack_ptr);

    //     let bytes = &mut self.ram.bytes;
    //     let mut available_mem = &mut bytes[start..];
    //     let (mut instr_blk_alloc, _) = available_mem.split_at_mut(start);
    //     instr_blk_alloc[start..end].clone_from_slice(&file_bytes[..]);
    //     let instr_block = InstructionBlock::new(&instr_blk_alloc[start..end])?;

    //     Ok(instr_block)
    // }

    fn exec_program(&mut self,
                    instr_blk: &InstructionBlock,
                    proc_contr_blk: &mut ProcessControlBlock) {
        loop {
            // Load instruction_ptr
            let instruction_ptr = self.cpu.instruction_ptr;
            // Increment instruction_ptr
            self.cpu.instruction_ptr += 1;
            let instr = instr_blk.get_instruction_at(instruction_ptr);
            // Execute instruction at instruction_ptr
            self.exec_instr(&instr);
        }
    }

    fn exec_instr(&mut self, instr: &Instruction) {
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

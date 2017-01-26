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

    pub fn exec(&mut self, file: &str, dump_each_time: bool) -> Result<String, String> {
        // Determine alloc sizes
        // /////////////////////////////////////////////////////////////////////////////////////////
        let file_bytes = self.fs.open_bytes_as_vec(file)?;
        let instr_blk_len = file_bytes.len();

        let stack_ptr = self.ram.get_stack_ptr();

        let instr_blk_start = stack_ptr as usize;

        let proc_contr_blk_start = instr_blk_start + instr_blk_len;

        let new_stack_ptr = proc_contr_blk_start + PCB_LEN;
        self.ram.set_stack_ptr(new_stack_ptr as u16);

        let mut available_mem = &mut self.ram.bytes[instr_blk_start..];

        // Create instruction block
        // /////////////////////////////////////////////////////////////////////////////////////////
        // TODO: maybe split into functions through the split
        let (mut instr_blk_alloc, mut available_mem) = available_mem.split_at_mut(instr_blk_len);
        instr_blk_alloc.clone_from_slice(&file_bytes);
        let instr_blk = InstructionBlock::new(instr_blk_alloc)?;

        // Create process control block
        // /////////////////////////////////////////////////////////////////////////////////////////
        let (mut proc_contr_blk_alloc, _) = available_mem.split_at_mut(PCB_LEN);

        let mut pcb = ProcessControlBlock::new(proc_contr_blk_alloc, 1, instr_blk_start as u16)?;
        self.cpu.instr_ptr = pcb.get_instr_ptr() as usize;
        let mut stack = pcb.get_stack_mut();

        // Execute code
        // /////////////////////////////////////////////////////////////////////////////////////////
        let mut output = String::new();
        let mut running = true;
        while running {
            use super::instruction::INSTRUCTION_SIZE;
            use super::instruction::InstructionType::*;

            if dump_each_time {
                print!("---\nCORE: {:?}\n", self.cpu);
                print!("STACK: {:?}\n---\n", stack.iter().map(|byte| *byte as char).collect::<Vec<char>>());
                // super::super::io_utils::wait_for_enter();
            }

            // Load instr_ptr
            let instr_ptr = self.cpu.instr_ptr;
            // Increment instr_ptr
            self.cpu.instr_ptr += INSTRUCTION_SIZE;
            let instr = instr_blk.get_instruction_at(instr_ptr);
            println!("{:?} {:?}", instr.get_type(), instr);
            // Execute instruction at instr_ptr
            match instr.get_type() {
                Load => {
                    let dest_reg = instr.get_reg_1() as usize;
                    let addr = instr.get_literal_2() as usize;
                    let mut reg_slice = self.cpu.registers.as_mut();
                    reg_slice[dest_reg] = byte_utils::get_u16_at(stack, addr);
                }
                LoadConstant => {
                    let dest_reg = instr.get_reg_1() as usize;
                    let constant = instr.get_literal_2();
                    let mut reg_slice = self.cpu.registers.as_mut();
                    reg_slice[dest_reg] = constant;
                }
                Store => {
                    let addr = instr.get_literal_1() as usize;
                    let src_reg = instr.get_reg_3() as usize;
                    let reg_slice = self.cpu.registers.as_mut();
                    byte_utils::set_u16_at(stack, addr, reg_slice[src_reg]);
                }
                Add => {
                    let src_reg_a = instr.get_reg_1() as usize;
                    let src_reg_b = instr.get_reg_2() as usize;
                    let dest_reg = instr.get_reg_3() as usize;
                    let mut reg_slice = self.cpu.registers.as_mut();
                    println!("{} + {} = {}", reg_slice[src_reg_a], reg_slice[src_reg_b], reg_slice[src_reg_a] + reg_slice[src_reg_b]);
                    reg_slice[dest_reg] = reg_slice[src_reg_a] + reg_slice[src_reg_b];
                }
                Subtract => {
                    let src_reg_a = instr.get_reg_1() as usize;
                    let src_reg_b = instr.get_reg_2() as usize;
                    let dest_reg = instr.get_reg_3() as usize;
                    let mut reg_slice = self.cpu.registers.as_mut();
                    reg_slice[dest_reg] = reg_slice[src_reg_a] - reg_slice[src_reg_b];
                }
                Multiply => {
                    let src_reg_a = instr.get_reg_1() as usize;
                    let src_reg_b = instr.get_reg_2() as usize;
                    let dest_reg = instr.get_reg_3() as usize;
                    let mut reg_slice = self.cpu.registers.as_mut();
                    reg_slice[dest_reg] = reg_slice[src_reg_a] * reg_slice[src_reg_b];
                }
                Divide => {
                    let src_reg_a = instr.get_reg_1() as usize;
                    let src_reg_b = instr.get_reg_2() as usize;
                    let dest_reg = instr.get_reg_3() as usize;
                    let mut reg_slice = self.cpu.registers.as_mut();
                    reg_slice[dest_reg] = reg_slice[src_reg_a] / reg_slice[src_reg_b];
                }
                Equal => {
                    let src_reg_a = instr.get_reg_1() as usize;
                    let src_reg_b = instr.get_reg_2() as usize;
                    let dest_reg = instr.get_reg_3() as usize;
                    let mut reg_slice = self.cpu.registers.as_mut();
                    let dest_val = if reg_slice[src_reg_a] == reg_slice[src_reg_b] {
                        0x01
                    } else {
                        0x00
                    };
                    reg_slice[dest_reg] = dest_val;
                }
                Goto => {
                    let addr = instr.get_literal_1() as usize;
                    if addr % INSTRUCTION_SIZE != 0 {
                        panic!("SEG_FAULT at setting IP to 0x{:x}", addr);
                    }
                    self.cpu.instr_ptr = addr;
                }
                GotoIf => {
                    let addr = instr.get_literal_1() as usize;
                    if addr % INSTRUCTION_SIZE != 0 {
                        panic!("SEG_FAULT at setting IP to 0x{:x}", addr);
                    }
                    let reg = instr.get_reg_3() as usize;
                    let reg_slice = self.cpu.registers.as_ref();
                    if reg_slice[reg] != 0 {
                        self.cpu.instr_ptr = addr;
                    }
                }
                // TODO: use the R, W types in Shell
                CharPrint => {
                    let addr = instr.get_literal_1() as usize;
                    let ascii_byte = stack[addr];
                    let ascii_char = ascii_byte as char;
                    output.push(ascii_char);
                    println!("CPRINT {:?}", ascii_char);
                }
                CharRead => {
                    use super::super::io_utils;
                    let addr = instr.get_literal_1() as usize;
                    let read_byte = io_utils::read_byte_from_stdin();
                    stack[addr] = read_byte;
                }
                Exit => running = false,
            }
        }

        Ok(output)
        // TODO: unalloc mem
    }

    // fn exec_instr(&mut self, instr: &Instruction) {
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
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] - reg_slice[src_reg_b];
    // }

    // fn mul(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] * reg_slice[src_reg_b];
    // }

    // fn div(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     reg_slice[dest_reg] = reg_slice[src_reg_a] / reg_slice[src_reg_b];
    // }

    // fn eq(&mut self, src_reg_a: usize, src_reg_b: usize, dest_reg: usize) {
    //     let mut reg_slice = self.cpu.registers.as_mut();
    //     let dest_val = if reg_slice[src_reg_a] == reg_slice[src_reg_b] {
    //         0x01
    //     } else {
    //         0x00
    //     };
    //     reg_slice[dest_reg] = dest_val;
    // }

    // fn goto(&mut self, addr: u32) {
    //     self.cpu.instr_ptr = addr;
    // }

    // fn goto_if(&mut self, addr: usize, reg: usize) {
    //     let reg_slice = self.cpu.registers.as_ref();
    //     if reg_slice[reg] == 0 {
    //         self.cpu.instr_ptr = addr;
    //     }
    // }
}

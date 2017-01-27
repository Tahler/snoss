use super::super::byte_utils;

use super::cpu::Cpu;
use super::storage::FileSystem;
use super::memory::Memory;
use super::instruction::InstructionBlock;
use super::process::ProcessControlBlock;

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

    pub fn exec(&mut self, file_name: &str, dump_each_time: bool) -> Result<String, String> {
        use super::process::PCB_LEN;

        // Determine alloc sizes
        // /////////////////////////////////////////////////////////////////////////////////////////
        let file_bytes = match self.fs.open_bytes_as_vec(file_name) {
            Ok(vec) => Ok(vec),
            Err(err) => Err(err.to_string()),
        }?;
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

        // Execute code
        // /////////////////////////////////////////////////////////////////////////////////////////
        let mut output = String::new();
        let mut running = true;
        let mut last_result: Result<(), ()> = Ok(());
        while running {
            use super::instruction::INSTRUCTION_LEN;
            use super::instruction::InstructionType::*;

            if dump_each_time {
                println!("{}", get_core_dump_str(&instr_blk, &self.cpu, &pcb));
            }

            // Load instr_ptr
            let instr_ptr = self.cpu.instr_ptr;
            // Increment instr_ptr
            self.cpu.instr_ptr += INSTRUCTION_LEN;
            last_result = if let Ok(instr) = instr_blk.get_instruction_at(instr_ptr) {
                let mut stack = pcb.get_stack_mut();

                // Execute instruction at instr_ptr
                match instr.get_type() {
                    Load => {
                        let dest_reg = instr.get_reg_1() as usize;
                        let addr = instr.get_literal_2() as usize;
                        if let Ok(loaded_val) = byte_utils::get_u16_at(stack, addr) {
                            self.cpu.set_reg(dest_reg, loaded_val)
                        } else {
                            Err(())
                        }
                    }
                    LoadConstant => {
                        let dest_reg = instr.get_reg_1() as usize;
                        let constant = instr.get_literal_2();
                        self.cpu.set_reg(dest_reg, constant)
                    }
                    Store => {
                        let addr = instr.get_literal_1() as usize;
                        let src_reg = instr.get_reg_3() as usize;
                        self.cpu
                            .get_reg(src_reg)
                            .and_then(|reg_val| byte_utils::set_u16_at(stack, addr, reg_val))
                    }
                    Add => {
                        let src_reg_a = instr.get_reg_1() as usize;
                        let src_reg_b = instr.get_reg_2() as usize;
                        let dest_reg = instr.get_reg_3() as usize;
                        let res_a = self.cpu.get_reg(src_reg_a);
                        let res_b = self.cpu.get_reg(src_reg_b);
                        if let (Ok(a), Ok(b)) = (res_a, res_b) {
                            self.cpu.set_reg(dest_reg, a + b)
                        } else {
                            Err(())
                        }
                    }
                    Subtract => {
                        let src_reg_a = instr.get_reg_1() as usize;
                        let src_reg_b = instr.get_reg_2() as usize;
                        let dest_reg = instr.get_reg_3() as usize;
                        let res_a = self.cpu.get_reg(src_reg_a);
                        let res_b = self.cpu.get_reg(src_reg_b);
                        if let (Ok(a), Ok(b)) = (res_a, res_b) {
                            self.cpu.set_reg(dest_reg, a - b)
                        } else {
                            Err(())
                        }
                    }
                    Multiply => {
                        let src_reg_a = instr.get_reg_1() as usize;
                        let src_reg_b = instr.get_reg_2() as usize;
                        let dest_reg = instr.get_reg_3() as usize;
                        let res_a = self.cpu.get_reg(src_reg_a);
                        let res_b = self.cpu.get_reg(src_reg_b);
                        if let (Ok(a), Ok(b)) = (res_a, res_b) {
                            self.cpu.set_reg(dest_reg, a * b)
                        } else {
                            Err(())
                        }
                    }
                    Divide => {
                        let src_reg_a = instr.get_reg_1() as usize;
                        let src_reg_b = instr.get_reg_2() as usize;
                        let dest_reg = instr.get_reg_3() as usize;
                        let res_a = self.cpu.get_reg(src_reg_a);
                        let res_b = self.cpu.get_reg(src_reg_b);
                        if let (Ok(a), Ok(b)) = (res_a, res_b) {
                            self.cpu.set_reg(dest_reg, a / b)
                        } else {
                            Err(())
                        }
                    }
                    Equal => {
                        let src_reg_a = instr.get_reg_1() as usize;
                        let src_reg_b = instr.get_reg_2() as usize;
                        let dest_reg = instr.get_reg_3() as usize;
                        let res_a = self.cpu.get_reg(src_reg_a);
                        let res_b = self.cpu.get_reg(src_reg_b);
                        if let (Ok(a), Ok(b)) = (res_a, res_b) {
                            let dest_val = if a == b { 0x01 } else { 0x00 };
                            self.cpu.set_reg(dest_reg, dest_val)
                        } else {
                            Err(())
                        }
                    }
                    Goto => {
                        let addr = instr.get_literal_1() as usize;
                        Ok(self.cpu.instr_ptr = addr)
                    }
                    GotoIf => {
                        let reg = instr.get_reg_3() as usize;
                        if let Ok(eq_val) = self.cpu.get_reg(reg) {
                            if eq_val != 0 {
                                let addr = instr.get_literal_1() as usize;
                                self.cpu.instr_ptr = addr;
                            }
                            Ok(())
                        } else {
                            Err(())
                        }
                    }
                    // TODO: use the R, W types in Shell
                    CharPrint => {
                        let addr = instr.get_literal_1() as usize;
                        stack.get(addr)
                            .map(|ascii_byte| *ascii_byte as char)
                            .map(|ascii_char| output.push(ascii_char))
                            .ok_or(())
                    }
                    CharRead => {
                        use super::super::io_utils;
                        let addr = instr.get_literal_1() as usize;
                        let read_byte = io_utils::read_byte_from_stdin();
                        stack.get_mut(addr)
                            .map(|slot| *slot = read_byte)
                            .ok_or(())
                    }
                    Exit => Ok(running = false),
                }
            } else {
                Err(())
            }
        }

        // TODO: dealloc mem

        match last_result {
            Ok(_) => Ok(output),
            Err(_) => {
                // TODO: dump core to file
                Err("Seg faulted".to_string())
            }
        }
    }

    // fn core_dump_on_seg_fault() -> RetType {
    //     unimplemented!();
    // }

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

fn get_core_dump_str(instr_blk: &InstructionBlock, cpu: &Cpu, pcb: &ProcessControlBlock) -> String {
    format!("{:?}\n{:?}\n{:?}\n", instr_blk, cpu, pcb)
}

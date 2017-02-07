use super::super::byte_utils;

use super::cpu::Cpu;
use super::fs::FileSystem;
use super::mem::Ram;
use super::instr::InstructionBlock;
use super::ps::Pcb;

pub const NUM_REGISTERS: usize = 6;
pub const RAM_LEN: usize = 10_000;

const CORE_DUMP_FILE_NAME: &'static str = "coredump";

#[derive(Debug)]
pub struct System {
    cpu: Cpu,
    ram: Ram,
    fs: FileSystem,
}

impl System {
    pub fn init() -> Self {
        System {
            cpu: Cpu::init(),
            ram: Ram::init(),
            fs: FileSystem::new("./fs"),
        }
    }

    pub fn list_files(&self) -> String {
        self.fs.list_files()
    }

    fn load_instr(&self, file_name: &str) -> Result<InstructionBlock, String> {
        let file_bytes = match self.fs.open_bytes_as_vec(file_name) {
            Ok(vec) => Ok(vec),
            Err(err) => Err(err.to_string()),
        }?;
        Ok(InstructionBlock::new(&file_bytes)?)
    }

    pub fn exec(&mut self, file_name: &str, dump_each_time: bool) -> Result<String, String> {
        let instr_blk = self.load_instr(file_name)?;
        let proc_id = self.ram.alloc_pcb(instr_blk);
        let result = {
            let mut pcb = self.ram.get_pcb_mut(proc_id);
            self.cpu.instr_ptr = pcb.get_instr_ptr();
            let instr_blk = &pcb.instr;

            let mut running = true;
            let mut output = String::new();
            let mut last_result: Result<(), ()> = Ok(());
            while running {
                use super::instr::INSTRUCTION_LEN;
                use super::instr::InstructionType::*;

                if dump_each_time {
                    println!("{}", get_core_dump_str(&self.cpu, &pcb));
                }

                // Load instr_ptr
                let instr_ptr = self.cpu.instr_ptr;
                // Increment instr_ptr
                self.cpu.instr_ptr += INSTRUCTION_LEN as u16;
                last_result = if let Ok(instr) = instr_blk.get_instruction_at(instr_ptr as usize) {
                    // Execute instruction at instr_ptr
                    match instr.get_type() {
                        Load => {
                            let dest_reg = instr.get_reg_1() as usize;
                            let addr = instr.get_literal_2() as usize;
                            let stack = &pcb.stack.bytes[..];
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
                            let stack = &mut pcb.stack.bytes[..];
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
                            Ok(self.cpu.instr_ptr = addr as u16)
                        }
                        GotoIf => {
                            let reg = instr.get_reg_3() as usize;
                            if let Ok(eq_val) = self.cpu.get_reg(reg) {
                                if eq_val != 0 {
                                    let addr = instr.get_literal_1() as usize;
                                    self.cpu.instr_ptr = addr as u16;
                                }
                                Ok(())
                            } else {
                                Err(())
                            }
                        }
                        CharPrint => {
                            let addr = instr.get_literal_1() as usize;
                            let stack = &pcb.stack.bytes[..];
                            stack.get(addr)
                                .map(|ascii_byte| *ascii_byte as char)
                                .map(|ascii_char| output.push(ascii_char))
                                .ok_or(())
                        }
                        CharRead => {
                            use super::super::io_utils;
                            let addr = instr.get_literal_1() as usize;
                            let read_byte = io_utils::read_byte_from_stdin();
                            let mut stack = &mut pcb.stack.bytes[..];
                            stack.get_mut(addr)
                                .map(|slot| *slot = read_byte)
                                .ok_or(())
                        }
                        Exit => Ok(running = false),
                    }
                } else {
                    Err(())
                };

                if last_result.is_err() {
                    running = false;
                }
            }

            match last_result {
                Ok(_) => Ok(output),
                Err(_) => {
                    self.fs.write_str_to_file(CORE_DUMP_FILE_NAME,
                                              &get_core_dump_str(&self.cpu, &pcb));
                    Err("Err: segfault".to_string())
                }
            }
        };

        self.ram.dealloc_pcb(proc_id);

        result
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

fn get_core_dump_str(cpu: &Cpu, pcb: &Pcb) -> String {
    format!("{:?}\n{:?}\n", cpu, pcb)
}

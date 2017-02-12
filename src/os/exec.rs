use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use byte_utils::{self, AccessResult};
use io_utils;
use time_utils;
use os::consts::TIME_SLICE_MS;
use super::cpu::Cpu;
use super::instr::Instruction;
use super::ps::Pcb;

/// Responsible for taking control of the CPU.
/// Each PCB should be "wrapped" with an executor to make it execute.
/// Can be thought of as a worker thread for a single process.
// #[derive(Debug)]
pub struct Executor {
    cpu: Arc<Mutex<Cpu>>,
    pcb: Arc<Mutex<Pcb>>,
    use_term: bool,
}

#[derive(Debug, PartialEq)]
pub enum ExecResult {
    Success,
    Exit,
    AccessErr,
}

impl Executor {
    pub fn new(cpu: Arc<Mutex<Cpu>>, pcb: Arc<Mutex<Pcb>>, use_term: bool) -> Executor {
        Executor {
            cpu: cpu.clone(),
            pcb: pcb.clone(),
            use_term: use_term,
        }
    }

    pub fn start(mut self, kill_tx: Sender<u16>) -> thread::JoinHandle<ExecResult> {
        thread::spawn(move || {
            let mut last_time_slice = time_utils::now();
            let mut result = ExecResult::Success;
            while result == ExecResult::Success {
                self.load_cpu_ctx();
                // TODO: should this line be below the yield?
                result = self.exec_once();

                let since_last_time_slice = time_utils::since(&last_time_slice);
                if since_last_time_slice.num_milliseconds() > TIME_SLICE_MS {
                    self.save_cpu_ctx();
                    // TODO: set pcb status
                    last_time_slice = time_utils::now();
                    thread::yield_now();
                }
            }
            let proc_id = self.get_proc_id();
            kill_tx.send(proc_id);
            result
        })
    }

    fn get_proc_id(&self) -> u16 {
        let pcb = self.pcb.lock().unwrap();
        pcb.get_id()
    }

    fn get_next_instr(&self) -> AccessResult<Instruction> {
        let cpu = self.cpu.lock().unwrap();
        let addr = cpu.instr_ptr as usize;
        let pcb = self.pcb.lock().unwrap();
        let instr_blk = pcb.get_instr_blk();
        Ok(*instr_blk.get_instruction_at(addr)?)
    }

    fn get_cpu_instr_ptr(&self) -> u16 {
        let cpu = self.cpu.lock().unwrap();
        cpu.instr_ptr
    }

    fn advance_cpu_instr_ptr(&mut self) {
        use super::instr::INSTRUCTION_LEN;
        let mut cpu = self.cpu.lock().unwrap();
        cpu.instr_ptr += INSTRUCTION_LEN as u16;
    }

    fn load_cpu_ctx(&mut self) {
        let mut cpu = self.cpu.lock().unwrap();
        let pcb = self.pcb.lock().unwrap();
        pcb.load_cpu_ctx(&mut cpu);
    }

    fn save_cpu_ctx(&mut self) {
        let cpu = self.cpu.lock().unwrap();
        let mut pcb = self.pcb.lock().unwrap();
        pcb.save_cpu_ctx(&cpu);
    }

    fn load(&mut self, dest_reg: u8, addr: u16) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        let mut pcb = self.pcb.lock().unwrap();
        let stack = pcb.get_stack_mut();
        byte_utils::get_u16_at(stack, addr as usize)
            .and_then(|loaded_val| cpu.set_reg(dest_reg, loaded_val))
    }

    fn load_const(&mut self, dest_reg: u8, constant: u16) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        cpu.set_reg(dest_reg, constant)
    }

    fn store(&mut self, src_reg: u8, addr: u16) -> AccessResult<()> {
        let mut pcb = self.pcb.lock().unwrap();
        let stack = pcb.get_stack_mut();
        let cpu = self.cpu.lock().unwrap();
        let addr = addr as usize;
        cpu.get_reg(src_reg)
            .and_then(|reg_val| byte_utils::set_u16_at(stack, addr, reg_val))
    }

    fn add(&mut self, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        let a = cpu.get_reg(src_reg_a)?;
        let b = cpu.get_reg(src_reg_b)?;
        cpu.set_reg(dest_reg, a + b)
    }

    fn sub(&mut self, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        let a = cpu.get_reg(src_reg_a)?;
        let b = cpu.get_reg(src_reg_b)?;
        cpu.set_reg(dest_reg, a - b)
    }

    fn mul(&mut self, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        let a = cpu.get_reg(src_reg_a)?;
        let b = cpu.get_reg(src_reg_b)?;
        cpu.set_reg(dest_reg, a * b)
    }

    fn div(&mut self, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        let a = cpu.get_reg(src_reg_a)?;
        let b = cpu.get_reg(src_reg_b)?;
        cpu.set_reg(dest_reg, a / b)
    }

    fn eq(&mut self, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
        let mut cpu = self.cpu.lock().unwrap();
        let a = cpu.get_reg(src_reg_a)?;
        let b = cpu.get_reg(src_reg_b)?;
        let res = if a == b { 0x01 } else { 0x00 };
        cpu.set_reg(dest_reg, res)
    }

    fn goto(&mut self, addr: u16) {
        let mut cpu = self.cpu.lock().unwrap();
        cpu.instr_ptr = addr;
    }

    fn goto_if(&mut self, if_reg: u8, addr: u16) -> AccessResult<()> {
        let eq_val = {
            let cpu = self.cpu.lock().unwrap();
            cpu.get_reg(if_reg)?
        };
        if eq_val != 0 {
            self.goto(addr);
        }
        Ok(())
    }

    fn char_print(&mut self, addr: u16) -> AccessResult<()> {
        if self.use_term {
            let mut pcb = self.pcb.lock().unwrap();
            let stack = pcb.get_stack_mut();
            let addr = addr as usize;
            let ch = stack.get(addr)
                .map(|ascii_byte| *ascii_byte as char)
                .ok_or(())?;
            let to_write = ch.to_string();
            print!("{}", to_write);
        }
        Ok(())
    }

    fn char_read(&mut self, addr: u16) -> AccessResult<()> {
        if self.use_term {
            let mut pcb = self.pcb.lock().unwrap();
            let mut stack = pcb.get_stack_mut();

            let addr = addr as usize;
            let read_byte = io_utils::read_byte_from_stdin();
            stack.get_mut(addr)
                .map(|slot| *slot = read_byte)
                .ok_or(())
        } else {
            panic!("Cannot run program requiring stdin async")
        }
    }

    fn dispatch(&mut self, instr: &Instruction) -> ExecResult {
        use super::instr::InstructionType::*;

        let reg_1 = instr.get_reg_1();
        let reg_2 = instr.get_reg_2();
        let reg_3 = instr.get_reg_3();
        let lit_1 = instr.get_literal_1();
        let lit_2 = instr.get_literal_2();

        match instr.get_type() {
            Load => self.load(reg_1, lit_2).into(),
            LoadConstant => self.load_const(reg_1, lit_2).into(),
            Store => self.store(reg_3, lit_1).into(),
            Add => self.add(reg_1, reg_2, reg_3).into(),
            Subtract => self.sub(reg_1, reg_2, reg_3).into(),
            Multiply => self.mul(reg_1, reg_2, reg_3).into(),
            Divide => self.div(reg_1, reg_2, reg_3).into(),
            Equal => self.eq(reg_1, reg_2, reg_3).into(),
            Goto => {
                self.goto(lit_1);
                ExecResult::Success
            }
            GotoIf => self.goto_if(reg_3, lit_1).into(),
            CharPrint => self.char_print(lit_1).into(),
            CharRead => self.char_read(lit_1).into(),
            Exit => ExecResult::Exit,
        }
    }

    pub fn exec_once(&mut self) -> ExecResult {
        let result = self.get_next_instr();
        let instr = match result {
            Ok(instr) => instr,
            Err(_) => return result.into(),
        };
        self.advance_cpu_instr_ptr();
        self.dispatch(&instr)
    }
}

impl<T> From<AccessResult<T>> for ExecResult {
    fn from(access_result: AccessResult<T>) -> ExecResult {
        match access_result {
            Ok(_) => ExecResult::Success,
            Err(_) => ExecResult::Exit,
        }
    }
}

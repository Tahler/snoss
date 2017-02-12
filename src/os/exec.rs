use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use byte_utils::{self, AccessResult};
use io_utils;
use time_utils;
use os::consts::TIME_SLICE_MS;
use super::cpu::Cpu;
use super::instr::Instruction;
use super::ps::{Pcb, Status as ProcessStatus};

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

    pub fn start(self, kill_tx: Sender<u16>) -> thread::JoinHandle<ExecResult> {
        thread::spawn(move || {
            let mut result = ExecResult::Success;
            while result == ExecResult::Success {
                {
                    use std::time::Duration;
                    // BEGIN TIME SLICE
                    let last_time_slice = time_utils::now();
                    // Rust does not seem to use FIFO queues for Mutex locks.
                    // This enables the other threads to grab hold of the resources.
                    thread::sleep(Duration::new(0, 1));
                    let mut cpu = self.cpu.lock().unwrap();
                    let mut pcb = self.pcb.lock().unwrap();
                    load_cpu_ctx(&mut cpu, &pcb);
                    // Execute
                    pcb.set_status(ProcessStatus::Executing);
                    while time_utils::since(&last_time_slice).num_milliseconds() <
                          TIME_SLICE_MS && result == ExecResult::Success {
                        result = exec_once(&mut cpu, &mut pcb, self.use_term);
                    }
                    // END TIME SLICE
                    save_cpu_ctx(&cpu, &mut pcb);
                    pcb.set_status(ProcessStatus::Blocked);
                }
                thread::yield_now();
            }
            let proc_id = self.get_proc_id();
            kill_tx.send(proc_id).unwrap();
            result
        })
    }

    fn get_proc_id(&self) -> u16 {
        let pcb = self.pcb.lock().unwrap();
        pcb.get_id()
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

fn get_next_instr(cpu: &Cpu, pcb: &Pcb) -> AccessResult<Instruction> {
    let addr = cpu.instr_ptr as usize;
    let instr_blk = pcb.get_instr_blk();
    // println!("loading instr at {}", addr);
    let instr = *instr_blk.get_instruction_at(addr)?;
    Ok(instr)
}

fn get_cpu_instr_ptr(cpu: &Cpu) -> u16 {
    cpu.instr_ptr
}

fn advance_cpu_instr_ptr(cpu: &mut Cpu) {
    use super::instr::INSTRUCTION_LEN;
    cpu.instr_ptr += INSTRUCTION_LEN as u16;
}

fn load_cpu_ctx(cpu: &mut Cpu, pcb: &Pcb) {
    pcb.load_cpu_ctx(cpu);
}

fn save_cpu_ctx(cpu: &Cpu, pcb: &mut Pcb) {
    pcb.save_cpu_ctx(&cpu);
}

fn load(cpu: &mut Cpu, pcb: &mut Pcb, dest_reg: u8, addr: u16) -> AccessResult<()> {
    let stack = pcb.get_stack_mut();
    byte_utils::get_u16_at(stack, addr as usize)
        .and_then(|loaded_val| cpu.set_reg(dest_reg, loaded_val))
}

fn load_const(cpu: &mut Cpu, dest_reg: u8, constant: u16) -> AccessResult<()> {
    cpu.set_reg(dest_reg, constant)
}

fn store(cpu: &Cpu, pcb: &mut Pcb, src_reg: u8, addr: u16) -> AccessResult<()> {
    let stack = pcb.get_stack_mut();
    let addr = addr as usize;
    cpu.get_reg(src_reg)
        .and_then(|reg_val| byte_utils::set_u16_at(stack, addr, reg_val))
}

fn add(cpu: &mut Cpu, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
    let a = cpu.get_reg(src_reg_a)?;
    let b = cpu.get_reg(src_reg_b)?;
    cpu.set_reg(dest_reg, a + b)
}

fn sub(cpu: &mut Cpu, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
    let a = cpu.get_reg(src_reg_a)?;
    let b = cpu.get_reg(src_reg_b)?;
    cpu.set_reg(dest_reg, a - b)
}

fn mul(cpu: &mut Cpu, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
    let a = cpu.get_reg(src_reg_a)?;
    let b = cpu.get_reg(src_reg_b)?;
    cpu.set_reg(dest_reg, a * b)
}

fn div(cpu: &mut Cpu, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
    let a = cpu.get_reg(src_reg_a)?;
    let b = cpu.get_reg(src_reg_b)?;
    cpu.set_reg(dest_reg, a / b)
}

fn eq(cpu: &mut Cpu, src_reg_a: u8, src_reg_b: u8, dest_reg: u8) -> AccessResult<()> {
    let a = cpu.get_reg(src_reg_a)?;
    let b = cpu.get_reg(src_reg_b)?;
    let res = if a == b { 0x01 } else { 0x00 };
    cpu.set_reg(dest_reg, res)
}

fn goto(cpu: &mut Cpu, addr: u16) {
    cpu.instr_ptr = addr;
}

fn goto_if(cpu: &mut Cpu, if_reg: u8, addr: u16) -> AccessResult<()> {
    let eq_val = cpu.get_reg(if_reg)?;
    if eq_val != 0 {
        goto(cpu, addr);
    }
    Ok(())
}

fn char_print(use_term: bool, pcb: &Pcb, addr: u16) -> AccessResult<()> {
    if use_term {
        let stack = pcb.get_stack();

        let addr = addr as usize;
        let ch = stack.get(addr)
            .map(|ascii_byte| *ascii_byte as char)
            .ok_or(())?;
        let to_write = ch.to_string();
        print!("{}", to_write);
    }
    Ok(())
}

fn char_read(use_term: bool, pcb: &mut Pcb, addr: u16) -> AccessResult<()> {
    if use_term {
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

fn dispatch(cpu: &mut Cpu, pcb: &mut Pcb, use_term: bool, instr: &Instruction) -> ExecResult {
    use super::instr::InstructionType::*;

    let reg_1 = instr.get_reg_1();
    let reg_2 = instr.get_reg_2();
    let reg_3 = instr.get_reg_3();
    let lit_1 = instr.get_literal_1();
    let lit_2 = instr.get_literal_2();

    match instr.get_type() {
        Load => load(cpu, pcb, reg_1, lit_2).into(),
        LoadConstant => load_const(cpu, reg_1, lit_2).into(),
        Store => store(cpu, pcb, reg_3, lit_1).into(),
        Add => add(cpu, reg_1, reg_2, reg_3).into(),
        Subtract => sub(cpu, reg_1, reg_2, reg_3).into(),
        Multiply => mul(cpu, reg_1, reg_2, reg_3).into(),
        Divide => div(cpu, reg_1, reg_2, reg_3).into(),
        Equal => eq(cpu, reg_1, reg_2, reg_3).into(),
        Goto => {
            goto(cpu, lit_1);
            ExecResult::Success
        }
        GotoIf => goto_if(cpu, reg_3, lit_1).into(),
        CharPrint => char_print(use_term, pcb, lit_1).into(),
        CharRead => char_read(use_term, pcb, lit_1).into(),
        Exit => ExecResult::Exit,
    }
}

fn exec_once(cpu: &mut Cpu, pcb: &mut Pcb, use_term: bool) -> ExecResult {
    let result = get_next_instr(&cpu, &pcb);
    let instr = match result {
        Ok(instr) => instr,
        Err(_) => return result.into(),
    };
    // println!("{} adv", pcb.get_id());
    advance_cpu_instr_ptr(cpu);
    dispatch(cpu, pcb, use_term, &instr)
}

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use super::cpu::Cpu;
use super::exec::{Executor, ExecResult};
use super::fs::FileSystem;
use super::instr::InstructionBlock;
use super::ps::{Pcb, Header as PcbHeader, ProcessTable};

pub mod consts {
    pub const NUM_REGISTERS: usize = 6;
    pub const RAM_LEN: usize = 10_000;
    pub const WORD_LEN: usize = 2;
    pub const STACK_LEN: usize = 64;
    pub const MAX_PROCS: usize = 10;
    pub const CORE_DUMP_FILE_NAME: &'static str = "coredump";
    pub const TIME_SLICE_MS: i64 = 1;
}

#[derive(Debug)]
pub struct System {
    // sched: Scheduler,
    cpu: Arc<Mutex<Cpu>>,
    proc_tbl: Arc<Mutex<ProcessTable>>,
    exit_tx: Sender<u16>,
    fs: FileSystem,
}

impl System {
    pub fn init() -> Self {
        let cpu = Arc::new(Mutex::new(Cpu::init()));
        let proc_tbl = ProcessTable::new();
        let proc_tbl = Arc::new(Mutex::new(proc_tbl));

        // Channel that informs threads of process completion.
        let (exit_tx, exit_rx): (Sender<u16>, Receiver<u16>) = mpsc::channel();
        let mut sys = System {
            cpu: cpu,
            proc_tbl: proc_tbl,
            exit_tx: exit_tx,
            fs: FileSystem::new("./fs"),
        };
        sys.listen_for_exit(exit_rx);
        sys
    }

    /// Spawns a "daemon" that removes processes from the process table.
    fn listen_for_exit(&mut self, exit_rx: Receiver<u16>) -> thread::JoinHandle<()> {
        let proc_tbl = self.proc_tbl.clone();
        thread::spawn(move || {
            loop {
                let exited_proc_id = exit_rx.recv().unwrap();
                let mut proc_tbl = proc_tbl.lock().unwrap();
                proc_tbl.dealloc_pcb(exited_proc_id);
            }
        })
    }

    pub fn list_files(&self) -> String {
        self.fs.list_files()
    }

    pub fn list_procs(&self) -> String {
        let header = format!("pid\tstate\tip\t1\t2\t3\t4\t5\t6\texe");
        let proc_tbl = self.proc_tbl.lock().unwrap();
        let procs = proc_tbl.get_running_procs();
        procs.map(|arc_pcb| {
                let pcb = arc_pcb.lock().unwrap();
                let Pcb { header: PcbHeader { id, ref exe_file_name, ref status, ref ctx }, .. } =
                    *pcb;
                let ip = ctx.instr_ptr;
                let reg = ctx.registers;
                let row = format!("{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
                                  id,
                                  status,
                                  ip,
                                  reg[0],
                                  reg[1],
                                  reg[2],
                                  reg[3],
                                  reg[4],
                                  reg[5],
                                  exe_file_name);
                row
            })
            .fold(header + "\n", |acc, row| acc + &row + "\n")
    }

    pub fn kill(&mut self, proc_id: u16) -> Result<(), String> {
        if self.proc_tbl.lock().unwrap().contains(proc_id) {
            self.exit_tx.send(proc_id);
            Ok(())
        } else {
            Err(format!("No process with {} exists.", proc_id))
        }
    }

    fn load_instr(&self, file_name: &str) -> Result<InstructionBlock, String> {
        let file_bytes = match self.fs.open_bytes_as_vec(file_name) {
            Ok(vec) => Ok(vec),
            Err(err) => Err(err.to_string()),
        }?;
        Ok(InstructionBlock::new(&file_bytes)?)
    }

    pub fn exec(&mut self,
                file_name: &str,
                use_term: bool)
                -> Result<thread::JoinHandle<ExecResult>, String> {
        let instr_blk = self.load_instr(file_name)?;
        let mut proc_tbl = self.proc_tbl.lock().unwrap();
        let proc_id = proc_tbl.alloc_pcb(file_name.to_string(), instr_blk)
            .ok_or("Could not allocate another process.".to_string())?;
        let pcb = proc_tbl.get_pcb(proc_id);
        let exec = Executor::new(self.cpu.clone(), pcb, use_term);
        Ok(exec.start(self.exit_tx.clone()))
    }
}

fn get_core_dump_str(cpu: &Cpu, pcb: &Pcb) -> String {
    format!("{:?}\n{:?}\n", cpu, pcb)
}

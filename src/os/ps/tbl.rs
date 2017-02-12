use std::collections::{self, HashMap, LinkedList};
use std::sync::{Arc, Mutex};
use os::consts::MAX_PROCS;
use super::pcb::Pcb;
use super::super::instr::InstructionBlock;

pub type PcbIter<'a> = collections::hash_map::Values<'a, u16, Arc<Mutex<Pcb>>>;
pub type PcbIterMut<'a> = collections::hash_map::ValuesMut<'a, u16, Arc<Mutex<Pcb>>>;

#[derive(Debug)]
pub struct ProcessTable {
    next_ids: LinkedList<u16>,
    // TODO: could easily use a fixed-sized array of blocks
    procs: HashMap<u16, Arc<Mutex<Pcb>>>,
}

impl ProcessTable {
    pub fn new() -> Self {
        use std::iter::FromIterator;
        ProcessTable {
            next_ids: LinkedList::from_iter((0..MAX_PROCS).map(|idx| idx as u16)),
            procs: HashMap::with_capacity(MAX_PROCS),
        }
    }

    pub fn contains(&self, proc_id: u16) -> bool {
        self.procs.contains_key(&proc_id)
    }

    pub fn get_pcb(&self, proc_id: u16) -> Arc<Mutex<Pcb>> {
        self.procs[&proc_id].clone()
    }

    pub fn get_running_procs(&self) -> PcbIter {
        self.procs.values()
    }

    pub fn get_running_procs_mut(&mut self) -> PcbIterMut {
        self.procs.values_mut()
    }

    /// Returns the Process ID of the allocated PCB.
    /// Returns `None` if there were no more available slots in the table.
    pub fn alloc_pcb(&mut self, instr: InstructionBlock) -> Option<u16> {
        self.next_ids.pop_front().map(|proc_id| {
            let pcb = Pcb::new(proc_id, instr);
            let pcb = Arc::new(Mutex::new(pcb));
            self.procs.insert(proc_id, pcb);
            proc_id
        })
    }

    pub fn dealloc_pcb(&mut self, proc_id: u16) {
        match self.procs.remove(&proc_id) {
            Some(_) => self.next_ids.push_front(proc_id),
            None => (),
        }
    }
}

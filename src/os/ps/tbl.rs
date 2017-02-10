use std::collections::HashMap;
use std::collections::LinkedList;
use os::consts::MAX_PROCS;
use super::pcb::Pcb;
use super::super::instr::InstructionBlock;

#[derive(Debug)]
pub struct ProcessTable {
    next_ids: LinkedList<u16>,
    procs: HashMap<u16, Pcb>,
}

impl ProcessTable {
    pub fn new() -> Self {
        use std::iter::FromIterator;
        ProcessTable {
            next_ids: LinkedList::from_iter((0..MAX_PROCS).map(|idx| idx as u16)),
            procs: HashMap::with_capacity(MAX_PROCS),
        }
    }

    pub fn get_pcb(&self, proc_id: u16) -> &Pcb {
        &self.procs[&proc_id]
    }

    pub fn get_pcb_mut(&mut self, proc_id: u16) -> &mut Pcb {
        self.procs.get_mut(&proc_id).unwrap()
    }

    // TODO: check num processes first, return Result
    /// Returns the Process ID of the allocated PCB.
    pub fn alloc_pcb(&mut self, instr: InstructionBlock) -> Option<u16> {
        self.next_ids.pop_front().map(|proc_id| {
            let pcb = Pcb::new(proc_id, instr);
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

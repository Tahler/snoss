use std::collections::HashSet;
use super::os::RAM_LEN;
use super::ps::Pcb;
use super::instr::InstructionBlock;

/// Number of concurrent processes
const BLOCK_LEN: usize = super::ps::PCB_LEN;
const NUM_BLOCKS: usize = RAM_LEN / BLOCK_LEN;

#[derive(Debug)]
pub struct Ram {
    /// Tracks the next open block
    next_avail: u16,
    /// Tracks the currently running processes
    procs: HashSet<u16>,
    block_tbl: Vec<Block>,
}

#[derive(Debug)]
enum Block {
    // Stores a linked-list-like next addr
    Avail(Option<u16>),
    Occ(Box<Pcb>),
}

impl Ram {
    pub fn init() -> Self {
        let mut blocks: Vec<Block> = Vec::with_capacity(NUM_BLOCKS);
        for _ in 0..NUM_BLOCKS {
            blocks.push(Block::Avail(None));
        }
        Ram {
            next_avail: 0,
            block_tbl: blocks,
            procs: HashSet::with_capacity(NUM_BLOCKS),
        }
    }

    pub fn get_pcb(&self, proc_id: u16) -> &Pcb {
        use self::Block::*;

        match self.block_tbl[proc_id as usize] {
            Avail(_) => panic!("No PCB for proc_id: {}", proc_id),
            Occ(ref boxed_pcb) => boxed_pcb,
        }
    }

    pub fn get_pcb_mut(&mut self, proc_id: u16) -> &mut Pcb {
        use self::Block::*;

        match self.block_tbl[proc_id as usize] {
            Avail(_) => panic!("No PCB for proc_id: {}", proc_id),
            Occ(ref mut boxed_pcb) => boxed_pcb,
        }
    }

    // TODO: check num processes first, return Result
    /// Returns the Process ID of the allocated PCB.
    pub fn alloc_pcb(&mut self, instr: InstructionBlock) -> u16 {
        use self::Block::*;

        let proc_id = self.next_avail;
        let slot_to_alloc = proc_id as usize;
        self.next_avail = match self.block_tbl[slot_to_alloc] {
            Avail(Some(n)) => n,
            Avail(None) => proc_id + 1,
            Occ(_) => panic!("Tried to allocate PCB in used block"),
        };
        let pcb = Pcb::new(proc_id, instr);
        self.block_tbl[slot_to_alloc] = Occ(Box::new(pcb));
        self.procs.insert(proc_id);
        proc_id
    }

    pub fn dealloc_pcb(&mut self, proc_id: u16) {
        use self::Block::*;

        let stored_next = self.next_avail;
        self.block_tbl[proc_id as usize] = Avail(Some(stored_next));
        self.procs.remove(&proc_id);
        self.next_avail = proc_id;
    }
}

impl Block {
    pub fn is_occ(&self) -> bool {
        match *self {
            Block::Occ(_) => true,
            _ => false,
        }
    }

    pub fn is_avail(&self) -> bool {
        match *self {
            Block::Avail(_) => true,
            _ => false,
        }
    }
}

use std::fmt;
use byte_utils::{self, AccessResult};
use super::{INSTRUCTION_LEN, Instruction};

pub const NUM_INSTRUCTIONS_PER_BLOCK: usize = 256;
pub const INSTRUCTION_BLOCK_LEN: usize = NUM_INSTRUCTIONS_PER_BLOCK * INSTRUCTION_LEN;

pub struct InstructionBlock {
    instructions: [Instruction; NUM_INSTRUCTIONS_PER_BLOCK],
}

impl InstructionBlock {
    pub fn new(bytes: &[u8]) -> Result<InstructionBlock, String> {
        if bytes.len() % INSTRUCTION_LEN == 0 {
            let mut instrs = [Instruction::from_word(0); NUM_INSTRUCTIONS_PER_BLOCK];

            let mut instr_bytes: [u8; 4] = [0; 4];
            for i in 0..bytes.len() {
                let byte_idx = i % INSTRUCTION_LEN;
                instr_bytes[byte_idx] = bytes[i];
                if byte_idx == INSTRUCTION_LEN - 1 {
                    let instr = Instruction::from_bytes(instr_bytes);
                    let instr_idx = i / INSTRUCTION_LEN;
                    instrs[instr_idx] = instr;
                    instr_bytes = [0; INSTRUCTION_LEN];
                }
            }
            let block = InstructionBlock { instructions: instrs };
            Ok(block)
        } else {
            Err(format!("An instruction block's size must be a multiple of the instruction size \
                         ({} bytes).",
                        INSTRUCTION_LEN))
        }
    }

    pub fn get_instruction_at(&self, addr: usize) -> AccessResult<&Instruction> {
        let idx = addr / INSTRUCTION_LEN;
        let is_in_bounds = idx < self.instructions.len();
        if is_aligned(addr) && is_in_bounds {
            Ok(&self.instructions[idx])
        } else {
            Err(())
        }
    }
}

fn is_aligned(addr: usize) -> bool {
    byte_utils::is_aligned(addr, INSTRUCTION_LEN)
}

impl fmt::Debug for InstructionBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "InstructionBlock: {{ instructions: {:?} }}",
               &self.instructions[..])
    }
}

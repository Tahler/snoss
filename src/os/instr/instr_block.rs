use byte_utils::{self, AccessResult};
use super::{INSTRUCTION_LEN, Instruction};

pub const NUM_INSTRUCTIONS_PER_BLOCK: usize = 256;
pub const INSTRUCTION_BLOCK_LEN: usize = NUM_INSTRUCTIONS_PER_BLOCK * INSTRUCTION_LEN;

#[derive(Debug)]
pub struct InstructionBlock {
    instructions: Vec<Instruction>,
}

impl InstructionBlock {
    pub fn new(bytes: &[u8]) -> Result<InstructionBlock, String> {
        if bytes.len() % INSTRUCTION_LEN == 0 {
            let mut instrs: Vec<Instruction> = Vec::with_capacity(bytes.len() / INSTRUCTION_LEN);
            let mut instr_bytes: [u8; 4] = [0; 4];
            for i in 0..bytes.len() {
                let instr_idx = i % INSTRUCTION_LEN;
                instr_bytes[instr_idx] = bytes[i];
                if instr_idx == INSTRUCTION_LEN - 1 {
                    let instr = Instruction::from_bytes(instr_bytes);
                    instrs.push(instr);
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

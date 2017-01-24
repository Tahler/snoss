mod byte_utils;

use std::iter::Iterator;

pub const INSTRUCTION_SIZE: usize = 4;

#[derive(Debug)]
pub struct Instruction {
    bytes: [u8; INSTRUCTION_SIZE],
}

impl Instruction {
    pub fn from_bytes(bytes: [u8; INSTRUCTION_SIZE]) -> Instruction {
        Instruction { bytes: bytes }
    }

    pub fn from_word(word: u32) -> Instruction {
        Instruction { bytes: byte_utils::u32_to_bytes(word) }
    }

    pub fn get_type(&self) -> Option<InstructionType> {
        match self.bytes[0] {
            0x11 => Some(InstructionType::Load),
            0x12 => Some(InstructionType::LoadConstant),
            0x13 => Some(InstructionType::Store),
            0x21 => Some(InstructionType::Add),
            0x22 => Some(InstructionType::Subtract),
            0x23 => Some(InstructionType::Multiply),
            0x24 => Some(InstructionType::Divide),
            0x25 => Some(InstructionType::Equal),
            0x31 => Some(InstructionType::Goto),
            0x32 => Some(InstructionType::GotoIf),
            0x41 => Some(InstructionType::CharPrint),
            0x42 => Some(InstructionType::CharRead),
            0xFF => Some(InstructionType::Exit),
            _ => None,
        }
    }

    pub fn get_reg_1(&self) -> u8 {
        self.bytes[1]
    }

    pub fn get_reg_2(&self) -> u8 {
        self.bytes[2]
    }

    pub fn get_reg_3(&self) -> u8 {
        self.bytes[3]
    }

    pub fn get_literal_1(&self) -> u16 {
        byte_utils::u16_from_bytes([self.bytes[1], self.bytes[2]])
    }

    pub fn get_literal_2(&self) -> u16 {
        byte_utils::u16_from_bytes([self.bytes[2], self.bytes[3]])
    }
}

#[derive(Debug)]
pub enum InstructionType {
    // Load / Store: 0x1N
    Load = 0x11,
    LoadConstant = 0x12,
    Store = 0x13,

    // Arithmetic: 0x2N
    Add = 0x21,
    Subtract = 0x22,
    Multiply = 0x23,
    Divide = 0x24,
    Equal = 0x25,

    // Goto: 0x3N
    Goto = 0x31,
    GotoIf = 0x32,

    // IO: 0x4N
    CharPrint = 0x41,
    CharRead = 0x42,

    Exit = 0xFF,
}

#[derive(Debug)]
pub struct InstructionBlock<'a> {
    mem_slice: &'a mut [u8],
}

impl<'a> InstructionBlock<'a> {
    pub fn new(mem_slice: &'a mut [u8]) -> Result<InstructionBlock<'a>, String> {
        if mem_slice.len() % INSTRUCTION_SIZE == 0 {
            let block = InstructionBlock { mem_slice: mem_slice };
            Ok(block)
        } else {
            Err(format!("An instruction block's size must be a multiple of the instruction size \
                         ({} bytes).",
                        INSTRUCTION_SIZE))
        }
    }

    pub fn get_instruction_at(&self, addr: usize) -> Instruction {
        let bytes = &self.mem_slice[addr..addr + INSTRUCTION_SIZE];
        Instruction::from_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

fn main() {
    let mut ram_stick = vec![0; 10].into_boxed_slice();
    ram_stick[0..4].clone_from_slice(&[0x12, 0x01, 0x00, 0x00]);

    {
        let instr_blk = InstructionBlock::new(&mut ram_stick[0..4]).unwrap();
        let instr_0 = instr_blk.get_instruction_at(0);
        println!("{:?}", instr_0.get_type());
    }

    println!("{:?}", ram_stick);
}

// TODO: probably not going to want to wrap the mutable slices.
// TODO: either explore splitting vectors (always owning the memory, no arrays)
// fn main() {
//     let mut ram_stick = vec![0; 10].into_boxed_slice();
//     let mut ram_block = Block::new(&mut ram_stick[..]);

//     {
//         let (mut block1, mut block2) = ram_block.split_at_mut(5);

//         block1.set_u16(0, 0xFF07);
//         block2.set_u16(0, 0x07FF);
//         println!("block1: {:?}", block1);
//         println!("block2: {:?}", block2);
//     }

//     println!("{:?}", ram_block);
// }

// fn main() {
//     let mut ram_stick = vec![0; 10].into_boxed_slice();

//     {
//         let (part1, part2) = ram_stick.split_at_mut(5);

//         let mut block1 = Block::new(part1);
//         block1.set_u16(0, 0xFF07);
//         let mut block2 = Block::new(part2);
//         block2.set_u16(0, 0x07FF);
//         println!("block1: {:?}", block1);
//         println!("block2: {:?}", block2);
//     }

//     println!("{:?}", ram_stick);
// }

// fn main() {
//     let mut ram_stick = vec![0; 10].into_boxed_slice();
//     {
//         let (part1, part2) = ram_stick.split_at_mut(5);
//         let mut block1 = Block::new(part1);
//         block1.set_u16(0, 0xFF07);
//         let block2 = Block::new(part2);
//         println!("block1: {:?}", block1);
//         println!("block2: {:?}", block2);
//     }
//     println!("{:?}", ram_stick);
// }

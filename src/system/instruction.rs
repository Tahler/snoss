use std::iter::Iterator;

use byte_utils;

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

// pub struct Instructions {
//     inner: Iterator<Item = u8>,
// }

// impl Iterator for Instructions {
//     type Item = Instruction;

//     fn next(&mut self) -> Option<Instruction> {
//         let inner = &mut self.inner;
//         let next4: Vec<u8> = inner.take(4)
//             // .map(|next| {
//             //     let byte: u8 = match next {
//             //         Some(b) => b,
//             //         e => panic!("{:?}", e), // TODO:
//             //     };
//             //     byte
//             // })
//             .collect();

//         match next4.len() {
//             4 => Some(Instruction::from_bytes(next4[0], next4[1], next4[2], next4[3])),
//             n if n > 4 => panic!("Somehow ended up taking more than 4 bytes. Took {}.", n),
//             _ /*if n < 4*/ => None,
//         }
//     }
// }

pub struct Instructions<'a> {
    inner: &'a [u8],
    ptr: usize,
}

impl<'a> Instructions<'a> {
    pub fn new(inner: &'a [u8]) -> Instructions<'a> {
        Instructions {
            inner: inner,
            ptr: 0,
        }
    }
}

impl<'a> Iterator for Instructions<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {
        use super::instruction::INSTRUCTION_SIZE;

        // TODO: err handling (continued next() calls)
        let instruction_start = self.ptr;
        let instruction_end = self.ptr + INSTRUCTION_SIZE;
        let next_bytes = &self.inner[instruction_start..instruction_end];

        self.ptr = instruction_end;

        match next_bytes.len() {
            n if n == INSTRUCTION_SIZE => {
                let mut arr = [0; INSTRUCTION_SIZE];
                arr.clone_from_slice(next_bytes);
                Some(Instruction::from_bytes(arr))
            },
            n if n > 4 => panic!("Somehow ended up taking more than 4 bytes. Took {}.", n),
            _ /*if n < 4*/ => None,
        }
    }
}

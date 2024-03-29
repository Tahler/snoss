use std::fmt;
use byte_utils;

pub const INSTRUCTION_LEN: usize = 4;

#[derive(Clone, Copy)]
pub struct Instruction {
    bytes: [u8; INSTRUCTION_LEN],
}

impl Instruction {
    pub fn from_bytes(bytes: [u8; INSTRUCTION_LEN]) -> Instruction {
        Instruction { bytes: bytes }
    }

    pub fn from_word(word: u32) -> Instruction {
        Instruction { bytes: byte_utils::u32_to_bytes(word) }
    }

    pub fn get_type(&self) -> InstructionType {
        use enum_primitive::FromPrimitive;

        let instr_byte = self.bytes[0];
        match InstructionType::from_u8(instr_byte) {
            Some(instr_type) => instr_type,
            None => panic!("Could not create InstructionType from {:?}", instr_byte),
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

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Instruction: {{ type: {:?}, bytes: {:?} }}",
               self.get_type(),
               self.bytes)
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
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
}

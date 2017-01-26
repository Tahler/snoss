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

    pub fn get_type(&self) -> InstructionType {
        use enum_primitive::FromPrimitive;

        let instr_byte = self.bytes[0];
        match InstructionType::from_u8(instr_byte) {
            Some(instr_type) => instr_type,
            None => panic!("Could not create InstructionType from {:?}.", instr_byte),
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

#[derive(Debug)]
pub struct InstructionBlock<'a> {
    mem_slice: &'a [u8],
}

impl<'a> InstructionBlock<'a> {
    pub fn new(mem_slice: &'a [u8]) -> Result<InstructionBlock<'a>, String> {
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

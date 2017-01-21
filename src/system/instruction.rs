#[derive(Debug)]
pub struct Instruction {
    byte_0: u8,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
}

impl Instruction {
    pub fn from_bytes(byte_0: u8, byte_1: u8, byte_2: u8, byte_3: u8) -> Instruction {
        Instruction {
            byte_0: byte_0,
            byte_1: byte_1,
            byte_2: byte_2,
            byte_3: byte_3,
        }
    }

    pub fn from_word(word: u32) -> Instruction {
        Instruction {
            byte_0: ((0xFF000000 & word) >> 24) as u8,
            byte_1: ((0x00FF0000 & word) >> 16) as u8,
            byte_2: ((0x0000FF00 & word) >> 08) as u8,
            byte_3: ((0x000000FF & word) >> 00) as u8,
        }
    }

    pub fn get_type(&self) -> Option<InstructionType> {
        match self.byte_0 {
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
        self.byte_1
    }

    pub fn get_reg_2(&self) -> u8 {
        self.byte_2
    }

    pub fn get_reg_3(&self) -> u8 {
        self.byte_3
    }

    pub fn get_literal_1(&self) -> u16 {
        let byte_1_shifted = (self.byte_1 as u16) << 8;
        let byte_2 = self.byte_2 as u16;
        byte_1_shifted | byte_2
    }

    pub fn get_literal_2(&self) -> u16 {
        let byte_2_shifted = (self.byte_2 as u16) << 8;
        let byte_3 = self.byte_3 as u16;
        byte_2_shifted | byte_3
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

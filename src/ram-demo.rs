mod byte_utils;

#[derive(Debug)]
pub struct Memory {
    bytes: Box<[u8]>,
}

impl Memory {
    pub fn new(bytes: Box<[u8]>) -> Self {
        Memory { bytes: bytes }
    }

    // pub fn alloc<'a>(&'a mut self, num_bytes: usize) -> Block<'a> {
    //     Block::new(&mut self.bytes[0..num_bytes])
    // }
}

#[derive(Debug)]
pub struct Block<'a> {
    mem_slice: &'a mut [u8],
}

impl<'a> Block<'a> {
    pub fn new(mem_slice: &'a mut [u8]) -> Block<'a> {
        Block { mem_slice: mem_slice }
    }

    pub fn split_at_mut(&'a mut self, addr: usize) -> (Block<'a>, Block<'a>) {
        let (mem_split_a, mem_split_b) = self.mem_slice.split_at_mut(addr);
        let block_a = Block::new(mem_split_a);
        let block_b = Block::new(mem_split_b);
        (block_a, block_b)
    }

    pub fn get_u8(&self, addr: usize) -> u8 {
        self.mem_slice[addr]
    }

    pub fn set_u8(&mut self, addr: usize, val: u8) {
        self.mem_slice[addr] = val;
    }

    pub fn get_u16(&self, addr: usize) -> u16 {
        let bytes = [self.mem_slice[addr], self.mem_slice[addr + 1]];
        byte_utils::u16_from_bytes(bytes)
    }

    pub fn set_u16(&mut self, addr: usize, val: u16) {
        let bytes = byte_utils::u16_to_bytes(val);
        let high_byte = bytes[0];
        self.set_u8(addr, high_byte);
        let low_byte = bytes[1];
        self.set_u8(addr + 1, low_byte);
    }

    pub fn len(&self) -> usize {
        self.mem_slice.len()
    }
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

fn main() {
    let mut ram_stick = vec![0; 10].into_boxed_slice();

    {
        let (part1, part2) = ram_stick.split_at_mut(5);

        let mut block1 = Block::new(part1);
        block1.set_u16(0, 0xFF07);
        let mut block2 = Block::new(part2);
        block2.set_u16(0, 0x07FF);
        println!("block1: {:?}", block1);
        println!("block2: {:?}", block2);
    }

    println!("{:?}", ram_stick);
}

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

#[derive(Debug)]
pub struct Memory {
    bytes: Box<[u8]>,
}

impl Memory {
    pub fn new(num_bytes: usize) -> Self {
        Memory { bytes: vec![0; num_bytes].into_boxed_slice() }
    }

    pub fn load(&self, addr: usize) -> u8 {
        let ram_slice = self.bytes.as_ref();
        ram_slice[addr]
    }

    pub fn store(&mut self, addr: usize, val: u8) {
        let mut ram_slice = self.bytes.as_mut();
        ram_slice[addr] = val;
    }
}

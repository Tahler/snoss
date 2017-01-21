use std::fs;
use std::io;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct FileSystem {
    mount_path: String,
}

impl FileSystem {
    pub fn new(mount_path: &str) -> Self {
        FileSystem { mount_path: mount_path.to_string() }
    }

    pub fn open(&self, file_name: &str) -> io::Result<fs::File> {
        let root = self.mount_path.to_string();
        let file_path = root + file_name;
        let file = fs::File::open(&file_path)?;
        Ok(file)
    }

    pub fn open_bytes(&self, file_path: &str) -> io::Result<io::Bytes<fs::File>> {
        use std::io::Read;

        let file = self.open(file_path)?;
        Ok(file.bytes())
    }

    pub fn open_instructions(&self, file_path: &str) -> io::Result<Instructions> {
        let bytes = self.open_bytes(file_path)?;
        Ok(Instructions { inner: bytes })
    }
}

pub struct Instructions {
    // TODO: more generic
    inner: io::Bytes<fs::File>,
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {
        let inner = &mut self.inner;
        let next4: Vec<u8> = inner
            .take(4)
            .map(|result| {
                let byte: u8 = match result {
                    Ok(b) => b,
                    e => panic!("{:?}", e), // TODO:
                };
                byte
            })
            .collect();

        match next4.len() {
            4 => Some(Instruction::from_bytes(next4[0], next4[1], next4[2], next4[3])),
            n if n > 4 => panic!("Somehow ended up taking more than 4 bytes. Took {}.", n),
            n /*if n < 4*/ => None,
        }
    }
}

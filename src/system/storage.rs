// TODO: err handling (no unwraps)

use std::fs;
use std::io;

#[derive(Debug)]
pub struct FileSystem {
    // TODO: use Path
    mount_path: String,
}

impl FileSystem {
    pub fn new(mount_path: &str) -> Self {
        let mut mount_path = mount_path.to_string();
        if !mount_path.ends_with('/') {
            mount_path.push('/')
        }
        FileSystem { mount_path: mount_path }
    }

    pub fn list_files(&self) -> String {
        let paths = fs::read_dir(&self.mount_path).unwrap();
        paths.map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
            .fold(String::new(), |mut acc, file_name| {
                acc.push_str(&file_name);
                acc.push('\t');
                acc
            })
    }

    pub fn open(&self, file_name: &str) -> io::Result<fs::File> {
        let root = self.mount_path.to_string();
        let file_path = root + file_name;
        let file = fs::File::open(&file_path)?;
        Ok(file)
    }

    pub fn open_bytes(&self, file_name: &str) -> Result<io::Bytes<fs::File>, String> {
        use std::io::Read;

        match self.open(file_name) {
            Ok(file) => Ok(file.bytes()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn open_bytes_as_vec(&self, file_name: &str) -> Result<Vec<u8>, String> {
        let mut bytes_iter = self.open_bytes(file_name)?;
        // TODO: does .any() consume it?
            Ok(bytes_iter.map(|result| result.unwrap())
                .collect())
        // if bytes_iter.any(|result| result.is_err()) {
        //     Err("BAD".to_string())
        // } else {
        //     Ok(bytes_iter.map(|result| result.unwrap())
        //         .collect())
        // }
    }
}

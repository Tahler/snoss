// TODO: err handling (no unwraps)

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct FileSystem {
    mount_path: Path,
}

impl FileSystem {
    pub fn new(mount_path: &str) -> Self {
        let mut mount_path = mount_path.to_string();
        if !mount_path.ends_with('/') {
            mount_path.push('/')
        }
        FileSystem { mount_path: mount_path }
    }

    fn get_full_path(&self, rel_path: AsRef<Path>) -> Path {
        self.mount_path.join(rel_path)
    }

    pub fn list_files(&self) -> String {
        let paths = fs::read_dir(self.mount_path).unwrap();
        paths.map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
            .fold(String::new(), |mut acc, file_name| {
                acc.push_str(&file_name);
                acc.push('\t');
                acc
            })
    }

    pub fn exists(&self, file_name: &str) -> bool {
        self.get_full_path(file_name).exists()
    }

    // pub fn create(file_name: &str) -> io::Result<()> {
    //     fs::File::create()
    // }

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
        Ok(self.open_bytes(file_name)?
            .map(|result| result.unwrap())
            .collect())
    }

    pub fn write_str_to_file(&self, file_name: &str, contents: &str) -> io::Result<()> {

    }
}

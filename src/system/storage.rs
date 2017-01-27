// TODO: err handling (no unwraps)

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileSystem {
    root: PathBuf,
}

impl FileSystem {
    pub fn new<P: AsRef<Path>>(mount_path: P) -> FileSystem {
        FileSystem { root: PathBuf::from(mount_path.as_ref()) }
    }

    fn get_full_path<P: AsRef<Path>>(&self, rel_path: P) -> PathBuf {
        self.root.join(rel_path)
    }

    pub fn list_files(&self) -> String {
        let paths = fs::read_dir(&self.root).unwrap();
        paths.map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
            .fold(String::new(), |acc, file_name| acc + &file_name + "\t")
    }

    pub fn exists<P: AsRef<Path>>(&self, file_name: P) -> bool {
        self.get_full_path(file_name).exists()
    }

    pub fn create<P: AsRef<Path>>(file_name: P) -> io::Result<fs::File> {
        fs::File::create(file_name)
    }

    pub fn open<P: AsRef<Path>>(&self, file_name: P) -> io::Result<fs::File> {
        let full_path = self.get_full_path(file_name);
        Ok(fs::File::open(full_path)?)
    }

    pub fn open_bytes(&self, file_name: &str) -> io::Result<io::Bytes<fs::File>> {
        use std::io::Read;

        Ok(self.open(file_name)?.bytes())
    }

    pub fn open_bytes_as_vec(&self, file_name: &str) -> io::Result<Vec<u8>> {
        let bytes = self.open_bytes(file_name)?;
        Ok(bytes.map(|result| result.unwrap())
            .collect())
    }

    pub fn write_str_to_file<P: AsRef<Path>>(&self,
                                             file_name: P,
                                             contents: &str)
                                             -> io::Result<()> {
        use std::io::Write;

        let mut file = self.open(file_name)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

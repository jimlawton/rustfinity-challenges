use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new<T: AsRef<str>>(file_name: T) -> Result<TempFile, io::Error> {
        let mut path = PathBuf::new();
        path.push(env::temp_dir());
        path.push(file_name.as_ref());
        fs::File::create(&path)?;
        Ok(TempFile { path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete temporary file: {}", e);
        }
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");
    assert!(tempfile.path.exists(), "File does not exist");
    drop(tempfile);
    assert!(!file_path.exists(), "File was not deleted");
    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");
    drop(tempfile_2);
}


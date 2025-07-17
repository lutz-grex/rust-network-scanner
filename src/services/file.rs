use std::{fs, io::Error, path::Path};


pub fn read_file(file_path: &Path) -> Result<String, Error> {
    fs::read_to_string(file_path)
}

pub fn write_file(file_path: &Path, content: String) -> Result<(), Error> {
    fs::write(file_path, content)
}

use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::io::prelude::*;

pub fn load_file_to_string(file_name: &str) -> Result<String, IoError> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}


use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::io::prelude::*;

pub fn load_file_to_string(file_name: &str, string: &mut String) -> Result<(), IoError> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(string)?;
    Ok(())
}

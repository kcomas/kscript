use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

pub fn read_file_to_string(file_name: &str) -> io::Result<String> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

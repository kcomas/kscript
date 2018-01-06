use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

pub fn load_file_to_string(name: &str) -> io::Result<String> {
    let file = File::open(name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

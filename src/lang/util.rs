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

#[derive(Debug)]
pub enum KscriptDebug {
    Stdout,
    File(String),
}

pub fn write_debug(title: &str, data: &str, location: &Option<KscriptDebug>) -> Result<(), String> {
    let out = match *location {
        Some(ref out) => out,
        None => return Err("Invalid debug object".to_string()),
    };
    let output_string = debug_string(title, data);
    match *out {
        KscriptDebug::Stdout => println!("{}", output_string),
        _ => {}
    };
    Ok(())
}

fn debug_string(title: &str, data: &str) -> String {
    format!(
        "{:-<10} {} {:-<10}\n{}\n{:-<10} End {} {:-<10}",
        "", title, "", data, "", title, ""
    )
}

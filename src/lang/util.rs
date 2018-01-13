use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};
use std::io::prelude::*;

pub fn load_file_to_string(name: &str) -> io::Result<String> {
    let file = File::open(name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn append_string_to_file(name: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(name)?;
    writeln!(file, "{}", data)?;
    Ok(())
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
        KscriptDebug::File(ref name) => {
            if let Err(error) = append_string_to_file(name, &output_string) {
                return Err(format!(
                    "Cannot appned debug info to file {} error: {:?}",
                    name, error
                ));
            }
        }
    };
    Ok(())
}

fn debug_string(title: &str, data: &str) -> String {
    format!(
        "{:-<10} {} {:-<10}\n{}\n{:-<10} End {} {:-<10}",
        "", title, "", data, "", title, ""
    )
}

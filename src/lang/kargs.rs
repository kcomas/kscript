use std::env;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum ArgFlags {
    Debug,
    DebugFile(String),
    Help,
}

enum ArgType {
    File,
    Flag(String, String),
}

#[derive(Debug)]
pub struct Kargs {
    pub zero: String,
    pub file: Option<String>,
    pub flags: Vec<ArgFlags>,
}

pub fn parse_args() -> Result<Kargs, String> {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    let mut file = None;
    let mut flags = Vec::new();
    let zero = match args.pop() {
        Some(zero) => zero,
        None => return Err("Cannot load 0th arg".to_string()),
    };

    while args.len() > 0 {
        let carg = args.pop().unwrap();
        let mut mabe_arg_type = None;
        {
            let mut iter = carg.chars().peekable();
            while let Some(c) = iter.next() {
                mabe_arg_type = match c {
                    '-' => load_flag(&mut iter),
                    _ => Some(ArgType::File),
                };
            }
        }

        if let Some(arg_type) = mabe_arg_type {
            match arg_type {
                ArgType::File => file = Some(carg),
                ArgType::Flag(key, value) => match key.as_str() {
                    "debug" => match value.as_str() {
                        "" => flags.push(ArgFlags::Debug),
                        _ => flags.push(ArgFlags::DebugFile(value)),
                    },
                    "help" => flags.push(ArgFlags::Help),
                    _ => {}
                },
            };
        }
    }

    Ok(Kargs {
        zero: zero,
        file: file,
        flags: flags,
    })
}

fn load_flag(iter: &mut Peekable<Chars>) -> Option<ArgType> {
    let mut key = String::new();
    let mut value = String::new();
    while let Some(c) = iter.next() {
        match c {
            '=' => break,
            _ => key.push(c),
        };
    }
    while let Some(c) = iter.next() {
        value.push(c);
    }
    Some(ArgType::Flag(key, value))
}

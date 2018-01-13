use std::env;

#[derive(Debug)]
pub struct ArgContainer {
    args: Vec<String>,
}

impl ArgContainer {
    pub fn new() -> ArgContainer {
        let mut args: Vec<String> = env::args().collect();
        args.reverse();
        ArgContainer { args: args }
    }

    pub fn has_args(&self) -> bool {
        self.args.len() > 0
    }

    pub fn next_arg(&mut self, error_message: &str) -> Result<String, String> {
        if let Some(arg) = self.args.pop() {
            return Ok(arg);
        }
        Err(error_message.to_string())
    }
}

#[derive(Debug)]
pub enum ArgFlags {
    Debug,
    DebugFile(String),
    Help,
}

#[derive(Debug)]
pub struct Kargs {
    pub zero: String,
    pub file: Option<String>,
    pub flags: Vec<ArgFlags>,
}

pub fn parse_args() -> Result<Kargs, String> {
    let mut arg_container = ArgContainer::new();
    let mut file = None;
    let mut flags = Vec::new();
    let zero = arg_container.next_arg("Cannot get 0th arg")?;

    while arg_container.has_args() {
        let next_arg = arg_container.next_arg("Cannot get next arg")?;
        match next_arg.as_str() {
            "-h" | "--help" => flags.push(ArgFlags::Help),
            "-d" | "--debug" => flags.push(ArgFlags::Debug),
            "-df" | "--debug-file" => flags.push(ArgFlags::DebugFile(arg_container
                .next_arg("Cannot get the debug file")?)),
            _ => file = Some(next_arg),
        };
    }

    Ok(Kargs {
        zero: zero,
        file: file,
        flags: flags,
    })
}

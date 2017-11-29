
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, DebugLogger, LoggerMode};
use std::{env, process};
use std::io::{self, Read};

struct Setup {
    use_logger: bool,
    logger_mode: LoggerMode,
    file: Option<String>,
    stdin: Option<String>,
    exec_name: String,
}

const HELP_ARGS: &'static str = "--help --log-stdout --log-stderr --read-stdin file.ks";

impl Setup {
    pub fn new() -> Setup {
        Setup {
            use_logger: false,
            logger_mode: LoggerMode::Void,
            file: None,
            stdin: None,
            exec_name: String::new(),
        }
    }

    pub fn get_logger(&self) -> Option<LoggerMode> {
        if self.use_logger {
            return Some(self.logger_mode.clone());
        }
        None
    }

    pub fn get_file(&self) -> Option<String> {
        self.file.clone()
    }

    pub fn get_exec_name(&self) -> &str {
        &self.exec_name
    }

    pub fn get_stdin(&self) -> Option<String> {
        self.stdin.clone()
    }

    pub fn get_args(&mut self, cli_args: &Vec<String>) {
        self.exec_name = cli_args[0].to_string();
        let total_args = cli_args.len();
        let mut current_arg = 0;
        while current_arg < total_args {
            let arg = cli_args[current_arg].as_ref();
            match arg {
                "--help" => {
                    help_print(self);
                    process::exit(1);
                }
                "--log-stdout" => {
                    self.use_logger = true;
                    self.logger_mode = LoggerMode::Stdout;
                }
                "--log--stderr" => {
                    self.use_logger = true;
                    self.logger_mode = LoggerMode::Stderr;
                }
                "--read-stdin" => {
                    let mut string = String::new();
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();
                    handle.read_to_string(&mut string).unwrap();
                    self.stdin = Some(string);
                }
                _ => {
                    self.file = Some(arg.to_string());
                }
            }
            current_arg += 1;
        }
    }
}

fn help_print(setup: &Setup) {
    println!("Usage: {} {}", setup.get_exec_name(), HELP_ARGS);
}

fn run<T: Logger>(kscript: &mut Kscript<T>, setup: &Setup) {
    if let Some(ref string) = setup.get_stdin() {
        if let Err(kerror) = kscript.run(string) {
            eprintln!("Error {:?}", kerror);
            process::exit(1);
        }
        process::exit(0);
    }

    if let Some(ref file) = setup.get_file() {
        if let Err(kerror) = kscript.run_file(file) {
            eprintln!("Error {:?}", kerror);
            process::exit(1);
        }
        process::exit(0);
    }
}

fn main() {
    let mut setup = Setup::new();
    let cli_args: Vec<String> = env::args().collect();
    setup.get_args(&cli_args);

    if let Some(logger_mode) = setup.get_logger() {
        let mut kscript = Kscript::new(DebugLogger::new(logger_mode));
        run(&mut kscript, &setup);
    } else {
        let mut kscript = Kscript::new(VoidLogger::new(LoggerMode::Void));
        run(&mut kscript, &setup);
    }

    help_print(&setup);
    process::exit(1);
}

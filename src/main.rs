
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, DebugLogger, LoggerMode};
use std::{env, process};

struct Setup {
    use_logger: bool,
    logger_mode: LoggerMode,
    file: Option<String>,
}

const HELP_MSG: &'static str = "Usage: kscript --help --log-stdout --log-stderr file.ks";

impl Setup {
    pub fn new() -> Setup {
        Setup {
            use_logger: false,
            logger_mode: LoggerMode::Void,
            file: None,
        }
    }

    pub fn get_args(&mut self, cli_args: &Vec<String>) {
        for arg in cli_args.iter() {
            match arg.as_ref() {
                "--help" => {
                    println!("{}", HELP_MSG);
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
                _ => {
                    self.file = Some(arg.to_string());
                }
            }
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
}

fn run<T: Logger>(kscript: &mut Kscript<T>, setup: &Setup) {
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

    println!("{}", HELP_MSG);
    process::exit(1);
}

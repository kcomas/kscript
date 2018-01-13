mod data_type;
mod command;
mod vm;
mod util;
mod builder;
mod error;
mod kargs;

use self::vm::Vm;
use self::util::{load_file_to_string, write_debug, KscriptDebug};
use self::command::SharedCommands;
use self::builder::{build_commands, SymbolTable};
use self::error::{KscriptError, ParserError};
use self::kargs::{parse_args, ArgFlags};

#[derive(Debug)]
pub struct Kscript {
    symbols: SymbolTable,
    commands: Option<SharedCommands>,
    vm: Vm,
    debug: Option<KscriptDebug>,
}

impl Kscript {
    pub fn new() -> Kscript {
        Kscript {
            symbols: SymbolTable::new(),
            commands: None,
            vm: Vm::new(),
            debug: None,
        }
    }

    pub fn set_debug(&mut self) {
        self.debug = Some(KscriptDebug::Stdout);
    }

    pub fn set_debug_file(&mut self, filename: &str) {
        self.debug = Some(KscriptDebug::File(filename.to_string()));
    }

    pub fn run_from_args(&mut self) -> Result<i32, KscriptError> {
        let kargs = match parse_args() {
            Ok(kargs) => kargs,
            Err(err) => return Err(KscriptError::CannotParseArgs(err)),
        };

        for arg in kargs.flags.iter() {
            match *arg {
                ArgFlags::Help => {}
                ArgFlags::Debug => self.set_debug(),
                ArgFlags::DebugFile(ref filename) => self.set_debug_file(filename),
            };
        }

        if let Some(ref filename) = kargs.file {
            return self.run_file(filename);
        }

        Ok(0)
    }

    pub fn run_file(&mut self, filename: &str) -> Result<i32, KscriptError> {
        let program = match load_file_to_string(filename) {
            Ok(program) => program,
            Err(file_error) => {
                return Err(KscriptError::ParserError(ParserError::FileLoadFile(
                    format!("{:?}", file_error),
                )))
            }
        };

        if self.debug.is_some() {
            write_debug("File String", &program, &self.debug).unwrap();
        }

        self.run_string(&program)
    }

    pub fn run_string(&mut self, program: &str) -> Result<i32, KscriptError> {
        let mut iter = program.chars().peekable();
        self.commands = match build_commands(&mut iter, &mut self.symbols, &self.debug) {
            Ok(commands) => Some(commands),
            Err(error) => return Err(KscriptError::ParserError(error)),
        };

        self.run()
    }

    fn run(&mut self) -> Result<i32, KscriptError> {
        let commands = match self.commands {
            Some(ref commands) => commands,
            None => return Err(KscriptError::VmCommandsEmpty),
        };

        let exit_code = match self.vm.run(commands) {
            Ok(exit_code) => exit_code,
            Err(error) => return Err(KscriptError::RuntimeError(error)),
        };

        if self.debug.is_some() {
            write_debug(
                "Exit Code",
                &format!("Exit Code: {}", exit_code),
                &self.debug,
            ).unwrap();
            write_debug("VM", &format!("{:#?}", self.vm), &self.debug).unwrap();
        }

        Ok(exit_code)
    }
}

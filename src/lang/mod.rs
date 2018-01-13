mod data_type;
mod command;
mod vm;
mod util;
mod builder;
mod error;

use self::vm::Vm;
use self::util::{load_file_to_string, write_debug, KscriptDebug};
use self::command::SharedCommands;
use self::builder::{build_commands, SymbolTable};
use self::error::{KscriptError, ParserError, RuntimeError};

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

    pub fn set_debug_stderr(&mut self) {
        self.debug = Some(KscriptDebug::Stderr);
    }

    pub fn set_debug_file(&mut self, filename: &str) {
        self.debug = Some(KscriptDebug::File(filename.to_string()));
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

        if self.debug.is_some() {
            write_debug("Commands", &format!("{:#?}", commands), &self.debug).unwrap();
        }

        let exit_code = match self.vm.run(commands) {
            Ok(exit_code) => exit_code,
            Err(error) => return Err(KscriptError::RuntimeError(error)),
        };

        if self.debug.is_some() {
            println!("Exit Code: {}", exit_code);
            println!("{:?}", self.vm);
        }

        Ok(exit_code)
    }
}

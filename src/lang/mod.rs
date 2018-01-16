mod data_type;
mod command;
mod vm;
mod util;
mod builder;
mod error;
mod kargs;

use std::io::{self, Write};
use self::vm::{CallInfo, Vm};
use self::util::{load_file_to_string, write_debug, KscriptDebug};
use self::command::SharedCommands;
use self::builder::{build_commands, SymbolTable};
use self::error::{KscriptError, ParserError};
use self::kargs::{help_message, parse_args, ArgFlags};

const REPL_INTRO: &str = "Kscript REPL, CTRL-D to exit";

#[derive(Debug)]
pub struct Kscript {
    symbols: SymbolTable,
    commands: Option<SharedCommands>,
    vm: Vm,
    vm_calls: Option<Vec<CallInfo>>,
    debug: Option<KscriptDebug>,
}

impl Kscript {
    pub fn new() -> Kscript {
        Kscript {
            symbols: SymbolTable::new(),
            commands: None,
            vm: Vm::new(),
            vm_calls: None,
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
                ArgFlags::Help => help_message(&kargs.zero),
                ArgFlags::Debug => self.set_debug(),
                ArgFlags::DebugFile(ref filename) => self.set_debug_file(filename),
            };
        }

        if let Some(ref filename) = kargs.file {
            return self.run_file(filename);
        }

        self.run_repl()
    }

    pub fn run_repl(&mut self) -> Result<i32, KscriptError> {
        let mut exit_code = 0;
        let mut input = String::new();
        println!("{}", REPL_INTRO);
        let stdin = io::stdin();
        let stdout = io::stdout();
        loop {
            {
                let mut stdout_lock = stdout.lock();
                stdout_lock
                    .write(b"& ")
                    .expect("Failed to write handle char");
                stdout_lock.flush().expect("Failed to flush handle char");
            }
            stdin.read_line(&mut input).expect("Could not read STDIN");
            if input.len() == 0 {
                println!("Exiting");
                break;
            }
            exit_code = match self.run_string(&input) {
                Ok(exit_code) => exit_code,
                Err(error) => {
                    println!("{:?}", error);
                    // reset the symbol table counter
                    if let Some(ref mut calls) = self.vm_calls {
                        if let Some(root_cals) = calls.first() {
                            self.symbols.set_counter(root_cals.locals.len());
                        }
                        while calls.len() > 1 {
                            // empty stack
                            calls.pop();
                        }
                    }
                    1
                }
            };
            input.clear();
        }
        Ok(exit_code)
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

        if let Some(ref mut calls) = self.vm_calls {
            calls.first_mut().unwrap().update_commands(commands);
        } else {
            self.vm_calls = Some(Vm::create_calls(commands));
        }

        let vm_calls = match self.vm_calls {
            Some(ref mut vm_calls) => vm_calls,
            None => return Err(KscriptError::CallDataEmpty),
        };

        let exit_code = match self.vm.run(vm_calls) {
            Ok(exit_code) => exit_code,
            Err(error) => return Err(KscriptError::RuntimeError(error)),
        };

        if self.debug.is_some() {
            write_debug(
                "Exit Code",
                &format!("Exit Code: {}", exit_code),
                &self.debug,
            ).unwrap();
            write_debug("Calls", &format!("{:#?}", vm_calls), &self.debug).unwrap();
            write_debug("VM", &format!("{:#?}", self.vm), &self.debug).unwrap();
        }

        Ok(exit_code)
    }
}

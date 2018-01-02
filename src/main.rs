mod util;
mod error;
mod ast;
mod data_type;
mod symbol;
mod command;
mod vm;

use std::{env, process};
use self::util::load_file_to_string;
use self::ast::load_ast;
use self::command::{load_commands, CommandState};
use self::symbol::SymbolTable;
use self::vm::Vm;
use self::error::Error;

fn printer(title: &str) {
    println!("{:-<10} {} {:-<10}", "", title, "");
}

fn handle_error(err: Error) {
    eprintln!("{:?}", err);
    eprintln!("Add KSCRIPT_DEBUG=1 for backtrace");
    process::exit(1);
}

fn main() {
    let kscript_debug = env::var("KSCRIPT_DEBUG").is_ok();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage {} file.ks", args[0]);
        process::exit(1);
    }
    let program = match load_file_to_string(&args[1]) {
        Ok(program) => program,
        Err(err) => return handle_error(Error::FileLoadFail(err)),
    };
    if kscript_debug {
        printer("Script");
        println!("{}", program);
        printer("End Script");
    }
    let mut iter = program.chars().peekable();
    let mut ast = match load_ast(&mut iter) {
        Ok(ast) => ast,
        Err(err) => return handle_error(err),
    };
    if kscript_debug {
        printer("AST");
        println!("{:#?}", ast);
        printer("End AST");
    }
    let mut commands = Vec::new();
    let mut root_symbols = SymbolTable::new();
    let mut command_state = CommandState::new(0);
    if let Err(err) = load_commands(
        &mut ast,
        &mut commands,
        &mut root_symbols,
        &mut command_state,
    ) {
        return handle_error(err);
    }
    if kscript_debug {
        printer("Symbols");
        println!("{:?}", root_symbols);
        printer("End Symbols");
        printer("VM Commands");
        println!("{:#?}", commands);
        printer("End VM");
    }
    let entry = match root_symbols.get_main() {
        Ok(entry) => entry,
        Err(err) => return handle_error(err),
    };
    if kscript_debug {
        println!("Entry: {}, {:?}", entry, commands[entry]);
    }
    let mut vm = Vm::new();
    match vm.run(&commands, entry) {
        Ok(exit_code) => {
            if kscript_debug {
                println!("{:?}", vm);
            }
            process::exit(exit_code);
        }
        Err(err) => return handle_error(err),
    };
}

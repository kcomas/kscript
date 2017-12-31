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
use self::command::load_commands;
use self::symbol::SymbolTable;
use self::vm::Vm;

fn main() {
    let kscript_debug = env::var("KSCRIPT_DEBUG").is_ok();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage {} file.ks", args[0]);
        process::exit(1);
    }
    let program = load_file_to_string(&args[1]).unwrap();
    if kscript_debug {
        println!("{}", program);
    }
    let mut iter = program.chars().peekable();
    let mut ast = load_ast(&mut iter).unwrap();
    if kscript_debug {
        println!("{:#?}", ast);
    }
    let mut commands = Vec::new();
    let mut root_symbols = SymbolTable::new();
    load_commands(&mut ast, &mut commands, &mut root_symbols).unwrap();
    if kscript_debug {
        println!("{:?}", root_symbols);
        println!("{:#?}", commands);
    }
    let entry = root_symbols.get_main().unwrap();
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
        Err(err) => {
            println!("{:?}", vm);
            println!("{:?}", err);
            println!("Add KSCRIPT_DEBUG=1 for backtrace");
        }
    }
}

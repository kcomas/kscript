mod util;
mod error;
mod ast;
mod data_type;
mod symbol;
mod command;
mod vm;

use self::util::load_file_to_string;
use self::ast::load_ast;
use self::command::load_commands;
use self::symbol::SymbolTable;
use self::vm::Vm;

fn main() {
    let program = load_file_to_string("./examples/ack.ks").unwrap();
    println!("{}", program);
    let mut iter = program.chars().peekable();
    let mut ast = load_ast(&mut iter).unwrap();
    println!("{:#?}", ast);
    let mut commands = Vec::new();
    let mut root_symbols = SymbolTable::new();
    load_commands(&mut ast, &mut commands, &mut root_symbols).unwrap();
    println!("{:?}", root_symbols);
    println!("{:#?}", commands);
    let entry = root_symbols.get_main().unwrap();
    println!("Entry: {}, {:?}", entry, commands[entry]);
    let mut vm = Vm::new();
    vm.run(&commands, entry).unwrap();
    println!("{:?}", vm);
}

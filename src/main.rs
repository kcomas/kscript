mod util;
mod error;
mod ast;
mod data_type;
mod symbol;
mod command;

use self::util::load_file_to_string;
use self::ast::load_ast;
use self::command::load_commands;
use self::symbol::SymbolTable;

fn main() {
    let program = load_file_to_string("./examples/fib.ks").unwrap();
    println!("{}", program);
    let mut iter = program.chars().peekable();
    let mut ast = load_ast(&mut iter).unwrap();
    println!("{:#?}", ast);
    let mut commands = Vec::new();
    let mut root_symbols = SymbolTable::new(true);
    load_commands(&mut ast, &mut commands, &mut root_symbols).unwrap();
    println!("{:?}", root_symbols);
}

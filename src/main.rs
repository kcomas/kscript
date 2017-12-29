mod util;
mod error;
mod ast;
mod data_type;
mod command;

use self::util::load_file_to_string;
use self::ast::load_ast;
use self::command::load_commands;

fn main() {
    let program = load_file_to_string("./examples/fib.ks").unwrap();
    println!("{}", program);
    let mut iter = program.chars().peekable();
    let mut ast = load_ast(&mut iter).unwrap();
    println!("{:#?}", ast);
    let commands = load_commands(&mut ast).unwrap();
}

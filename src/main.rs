mod data_type;
mod command;
mod vm;
mod util;
mod builder;
mod error;

use std::rc::Rc;
use self::data_type::DataType;
use self::command::Command;
use self::vm::Vm;
use self::util::load_file_to_string;
use self::builder::{build_commands, SymbolTable};

fn main() {
    let program = load_file_to_string("./examples/fib.ks").unwrap();
    println!("{}", program);
    let mut iter = program.chars().peekable();
    let mut root_symbols = SymbolTable::new();
    let commands = build_commands(&mut iter, &mut root_symbols).unwrap();
    println!("{:#?}", commands);
    let mut vm = Vm::new();

    let code = vm.run(&commands).unwrap();
    println!("Exit Code: {}", code);
    println!("{:?}", vm);
}

mod memory;
mod address;
mod function;
mod data;
mod command;
mod vm;
mod error;
mod util;
mod token;
mod lexer;
mod ast;

use std::path::Path;
use self::memory::Memory;
use self::data::Data;
use self::function::FunctionPointer;
use self::command::Command;
use self::vm::Vm;
use self::util::read_file_to_string;
use self::lexer::{create_matchers, lex};

pub fn run() {
    let file_string = read_file_to_string(Path::new("./examples/fib.ks")).unwrap();
    println!("{}", file_string);
    let mut iter = file_string.chars().peekable();
    let tokens = lex(&mut iter, &create_matchers()).unwrap();
    println!("{:#?}", tokens);

    let mut memory = Memory::new();
    let i1 = memory.insert_fixed(Data::Integer(15));
    let i2 = memory.insert_fixed(Data::Integer(0));
    let i3 = memory.insert_fixed(Data::Integer(0));
    let i4 = memory.insert_fixed(Data::Integer(1));
    let i5 = memory.insert_fixed(Data::Integer(1));
    let i6 = memory.insert_fixed(Data::Integer(1));
    let i7 = memory.insert_fixed(Data::Integer(2));
    let s1 = memory.insert_fixed(Data::String("Hello World".to_string()));

    let f1 = memory.insert_fixed(Data::Function(FunctionPointer {
        entry_index: 7,
        number_arguments: 1,
        number_locals: 0,
    }));

    let commands = vec![
        Command::Push(i1),
        Command::Push(f1),
        Command::Call,
        Command::Print,
        Command::Push(s1),
        Command::Print,
        Command::Halt(0),
        Command::LoadArg(0),
        Command::Push(i2),
        Command::Equals,
        Command::JumpIfFalse(3),
        Command::Push(i3),
        Command::Return,
        Command::LoadArg(0),
        Command::Push(i4),
        Command::Equals,
        Command::JumpIfFalse(3),
        Command::Push(i5),
        Command::Return,
        Command::LoadArg(0),
        Command::Push(i6),
        Command::Sub,
        Command::CallSelf,
        Command::LoadArg(0),
        Command::Push(i7),
        Command::Sub,
        Command::CallSelf,
        Command::Add,
        Command::Return,
    ];

    let mut vm = Vm::new(0, 0);

    let exit_code = vm.run(&mut memory, &commands).unwrap();
    println!("Exit Code {}", exit_code);
    println!("{:?}", vm);
    println!("{:?}", memory);
}

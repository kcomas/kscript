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
use self::builder::build_commands;

fn main() {
    let program = load_file_to_string("./examples/fib.ks").unwrap();
    let mut iter = program.chars().peekable();
    let c = build_commands(&mut iter).unwrap();
    println!("{:?}", c);

    let commands = Rc::new(vec![
        Command::PushStack(DataType::Function(
            Rc::new(vec![
                Command::LoadStackArg(0),
                Command::PushStack(DataType::Integer(0)),
                Command::Equals,
                Command::JumpIfFalse(3),
                Command::PushStack(DataType::Integer(0)),
                Command::Return,
                Command::LoadStackArg(0),
                Command::PushStack(DataType::Integer(1)),
                Command::Equals,
                Command::JumpIfFalse(3),
                Command::PushStack(DataType::Integer(1)),
                Command::Return,
                Command::LoadStackArg(0),
                Command::PushStack(DataType::Integer(1)),
                Command::Sub,
                Command::CallSelf,
                Command::LoadStackArg(0),
                Command::PushStack(DataType::Integer(2)),
                Command::Sub,
                Command::CallSelf,
                Command::Add,
                Command::Return,
            ]),
            1,
        )),
        Command::SaveLocal(0),
        Command::PushStack(DataType::Integer(15)),
        Command::LoadLocal(0),
        Command::Call,
        Command::Println,
        Command::Halt(0),
    ]);

    let mut vm = Vm::new();

    // let code = vm.run(&commands).unwrap();
    // println!("Exit Code: {}", code);
    // println!("{:?}", vm);
}

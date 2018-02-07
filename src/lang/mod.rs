mod data_type;
mod command;
mod vm;
mod error;

use self::data_type::{DataType, FunctionPointer};
use self::command::Command;
use self::vm::Vm;

pub fn run() {
    let commands = vec![
        Command::PushStack(DataType::create_integer(30)),
        Command::PushStack(DataType::create_function(FunctionPointer {
            command_index: 4,
            num_arguments: 1,
            num_locals: 0,
            length: 21,
        })),
        Command::Call,
        Command::Halt(0),
        Command::LoadArgument(0),
        Command::PushStack(DataType::create_integer(0)),
        Command::Equals,
        Command::JumpIfFalse(2),
        Command::PushStack(DataType::create_integer(0)),
        Command::Return,
        Command::LoadArgument(0),
        Command::PushStack(DataType::create_integer(1)),
        Command::Equals,
        Command::JumpIfFalse(2),
        Command::PushStack(DataType::create_integer(1)),
        Command::Return,
        Command::LoadArgument(0),
        Command::PushStack(DataType::create_integer(1)),
        Command::Sub,
        Command::CallSelf,
        Command::LoadArgument(0),
        Command::PushStack(DataType::create_integer(2)),
        Command::Sub,
        Command::CallSelf,
        Command::Add,
        Command::Return,
    ];

    let mut frames = Vm::create_frames();

    let mut vm = Vm::new();

    let exit_code = vm.run(0, &commands, &mut frames).unwrap();

    println!("Exit Code: {}", exit_code);
    println!("{:?}", vm);
}

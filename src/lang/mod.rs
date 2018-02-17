mod memory;
mod error;
mod data;
mod function;
mod command;
mod vm;

use self::memory::Memory;
use self::data::DataHolder;
use self::function::FunctionPointer;
use self::command::Command;
use self::vm::Vm;

pub fn run() {
    let mut memory = Memory::new();

    let i1 = memory.insert_fixed(DataHolder::Integer(30));
    let i2 = memory.insert_fixed(DataHolder::Integer(0));
    let i3 = memory.insert_fixed(DataHolder::Integer(0));
    let i4 = memory.insert_fixed(DataHolder::Integer(1));
    let i5 = memory.insert_fixed(DataHolder::Integer(1));
    let i6 = memory.insert_fixed(DataHolder::Integer(1));
    let i7 = memory.insert_fixed(DataHolder::Integer(2));

    let f1 = memory.insert_fixed(DataHolder::Function(FunctionPointer {
        current_command_index: 6,
        num_arguments: 2,
        num_locals: 0,
    }));

    let mut vm = Vm::new();
    vm.init(&mut memory, 0, 0);

    let commands = vec![
        Command::Push(i1),
        Command::Push(f1.clone()),
        Command::Push(f1),
        Command::Call,
        Command::Print,
        Command::Halt(0),
        Command::LoadArg(2),
        Command::Push(i2),
        Command::Equals,
        Command::JumpIfFalse(2),
        Command::Push(i3),
        Command::Return,
        Command::LoadArg(2),
        Command::Push(i4),
        Command::Equals,
        Command::JumpIfFalse(2),
        Command::Push(i5),
        Command::Return,
        Command::LoadArg(2),
        Command::Push(i6),
        Command::Sub,
        Command::LoadArg(1),
        Command::LoadArg(1),
        Command::Call,
        Command::LoadArg(2),
        Command::Push(i7),
        Command::Sub,
        Command::LoadArg(1),
        Command::LoadArg(1),
        Command::Call,
        Command::Add,
        Command::Return,
    ];

    let exit_code = vm.run(&mut memory, &commands).unwrap();

    println!("Exit Code {}", exit_code);
    println!("{:?}", vm);
}

mod memory;
mod address;
mod function;
mod data;
mod command;
mod vm;
mod error;

use self::memory::Memory;
use self::data::Data;
use self::function::FunctionPointer;
use self::command::Command;
use self::vm::Vm;

pub fn run() {
    let mut memory = Memory::new();
    let i1 = memory.insert_fixed(Data::Integer(30));
    let i2 = memory.insert_fixed(Data::Integer(0));
    let i3 = memory.insert_fixed(Data::Integer(0));
    let i4 = memory.insert_fixed(Data::Integer(1));
    let i5 = memory.insert_fixed(Data::Integer(1));
    let i6 = memory.insert_fixed(Data::Integer(1));
    let i7 = memory.insert_fixed(Data::Integer(2));

    let f1 = memory.insert_fixed(Data::Function(FunctionPointer {
        entry_index: 5,
        number_arguments: 1,
        number_locals: 0,
    }));

    let commands = vec![
        Command::Push(i1),
        Command::Push(f1),
        Command::Call,
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

    let mut vm = Vm::new(0);

    let exit_code = vm.run(&mut memory, &commands).unwrap();
    // println!("Exit Code {}", exit_code);
    // println!("{:?}", vm);
    // println!("{:?}", memory);
}

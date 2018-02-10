mod memory;
mod error;
mod function;
mod data;
mod command;
mod vm;

use self::memory::Memory;
use self::data::DataHolder;
use self::vm::Vm;
use self::command::Command;
use self::function::FunctionPointer;

pub fn run() {
    let mut memory = Memory::new();
    let i1 = memory.insert_fixed(DataHolder::Integer(30));
    let f1 = memory.insert_fixed(DataHolder::Function(FunctionPointer {
        entry_command_index: 5,
        current_command_index: 5,
        num_arguments: 1,
        num_locals: 0,
        entry_stack_len: 0,
    }));
    let i2 = memory.insert_fixed(DataHolder::Integer(0));
    let i3 = memory.insert_fixed(DataHolder::Integer(0));
    let i4 = memory.insert_fixed(DataHolder::Integer(1));
    let i5 = memory.insert_fixed(DataHolder::Integer(1));
    let i6 = memory.insert_fixed(DataHolder::Integer(1));
    let i7 = memory.insert_fixed(DataHolder::Integer(2));

    let commands = vec![
        Command::PushStack(i1),
        Command::PushStack(f1),
        Command::Call,
        Command::Print,
        Command::Halt(0),
        Command::LoadArgument(0),
        Command::PushStack(i2),
        Command::Equals,
        Command::JumpIfFalse(2),
        Command::PushStack(i3),
        Command::Return,
        Command::LoadArgument(0),
        Command::PushStack(i4),
        Command::Equals,
        Command::JumpIfFalse(2),
        Command::PushStack(i5),
        Command::Return,
        Command::LoadArgument(0),
        Command::PushStack(i6),
        Command::Sub,
        Command::CallSelf,
        Command::LoadArgument(0),
        Command::PushStack(i7),
        Command::Sub,
        Command::CallSelf,
        Command::Add,
        Command::Return,
    ];

    let mut call_stack = Vm::create_call_stack();
    let mut vm = Vm::new();

    let exit_code = vm.run(&commands, &mut memory, &mut call_stack).unwrap();
    // println!("{:?}", vm);
    // println!("{:?}", memory);
}

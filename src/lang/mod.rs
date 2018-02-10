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

pub fn run() {
    let mut memory = Memory::new();
    let i1 = memory.insert_fixed(DataHolder::Integer(3));
    let i2 = memory.insert_fixed(DataHolder::Integer(4));
    let i3 = memory.insert_fixed(DataHolder::Integer(2));

    let commands = vec![
        Command::PushStack(i1),
        Command::PushStack(i2),
        Command::Add,
        Command::PushStack(i3),
        Command::Sub,
        Command::Halt(0),
    ];

    let mut call_stack = Vm::create_call_stack();
    let mut vm = Vm::new();

    let exit_code = vm.run(&commands, &mut memory, &mut call_stack).unwrap();
    println!("{:?}", vm);
    println!("{:?}", memory);
}

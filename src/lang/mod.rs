mod memory;
mod error;
mod data;
mod function;
mod command;
mod vm;

use self::memory::Memory;
use self::data::DataHolder;
use self::command::Command;
use self::vm::Vm;

pub fn run() {
    let mut memory = Memory::new();

    let i1 = memory.insert_fixed(DataHolder::Integer(3));
    let i2 = memory.insert_fixed(DataHolder::Integer(4));

    let mut vm = Vm::new();
    vm.init(&mut memory, 0, 0);

    let commands = vec![Command::Push(i1), Command::Push(i2), Command::Halt(0)];

    let exit_code = vm.run(&mut memory, &commands).unwrap();

    println!("Exit Code {}", exit_code);
    println!("{:?}", vm);
}

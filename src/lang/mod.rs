mod memory;
mod address;
mod function;
mod data;
mod command;
mod vm;
mod error;

use self::memory::Memory;
use self::data::Data;
use self::command::Command;
use self::vm::Vm;

pub fn run() {
    let mut memory = Memory::new();
    let i1 = memory.insert_fixed(Data::Integer(3));
    let i2 = memory.insert_fixed(Data::Integer(4));
    let i3 = memory.insert_fixed(Data::Integer(2));

    let commands = vec![
        Command::Push(i1),
        Command::Push(i2),
        Command::Add,
        Command::Push(i3),
        Command::Sub,
        Command::Halt(0),
    ];

    let mut vm = Vm::new(0);

    let exit_code = vm.run(&mut memory, &commands).unwrap();
    println!("Exit Code {}", exit_code);
    println!("{:?}", vm);
    println!("{:?}", memory);
}

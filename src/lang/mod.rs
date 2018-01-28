mod command;
mod memory;
mod data;
mod vm;
mod error;

use self::command::Command;
use self::memory::{Function, Memory};
use self::data::DataHolder;
use self::vm::Vm;

pub fn run() {
    let mut memory = Memory::new();
    //    let i1 = memory.insert(DataHolder::Integer(1));
    //    let i2 = memory.insert(DataHolder::Integer(2));
    //    {
    //        let ref1 = memory.get(&i1);
    //        let ref2 = memory.get(&i2);
    //        println!("{:?}", ref1);
    //        println!("{:?}", ref2);
    //    }
    //    memory.update(&i1, DataHolder::Integer(15));
    //    let ref1 = memory.get(&i1);
    //    let ref2 = memory.get(&i2);
    //    println!("{:?}", ref1);
    //    println!("{:?}", ref2);

    let i1 = memory.insert(DataHolder::Integer(8), false);
    let i2 = memory.insert(DataHolder::Integer(0), true);
    let i3 = memory.insert(DataHolder::Integer(1), true);

    let f1 = memory.insert(
        DataHolder::Function(Function::new(
            vec![
                Command::LoadArgument(0),
                Command::PushStack(i2.clone()),
                Command::Equals,
                Command::JumpIfFalse(2),
                Command::PushStack(i2.clone()),
                Command::Return,
                Command::LoadArgument(0),
                Command::PushStack(i3.clone()),
                Command::Equals,
                Command::JumpIfFalse(2),
                Command::PushStack(i3.clone()),
                Command::Return,
                Command::LoadArgument(0),
                Command::Return,
            ],
            1,
        )),
        false,
    );

    let main_address = memory.insert(
        DataHolder::Function(Function::new(
            vec![
                Command::PushStack(i1),
                Command::PushStack(f1),
                Command::Call,
                Command::PrintDebug,
                Command::Halt(0),
            ],
            0,
        )),
        false,
    );

    let mut vm_calls = Vm::create_calls(main_address.get_address());

    let mut vm = Vm::new();

    let exit_code = vm.run(&mut memory, &mut vm_calls).unwrap();
    println!("Exit Code {}", exit_code);
    println!("{:?}", memory);
    println!("{:?}", vm);
}

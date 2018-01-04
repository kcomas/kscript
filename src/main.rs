mod data_type;
mod command;
mod vm;
mod error;

use std::rc::Rc;
use self::data_type::DataType;
use self::command::Command;
use self::vm::Vm;

fn main() {
    let commands = Rc::new(vec![
        Command::PushStack(DataType::Integer(0)),
        Command::PushStack(DataType::Function(
            Rc::new(vec![
                Command::PushStack(DataType::Integer(0)),
                Command::Return,
            ]),
            1,
        )),
        Command::Call,
        Command::Halt(0),
    ]);

    let mut vm = Vm::new();

    let code = vm.run(&commands).unwrap();
    println!("Exit Code: {}", code);
    println!("{:?}", vm);
}

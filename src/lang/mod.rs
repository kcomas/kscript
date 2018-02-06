mod data_type;
mod command;
mod vm;
mod error;

use self::data_type::DataType;
use self::command::Command;
use self::vm::Vm;

pub fn run() {
    let commands = vec![
        Command::PushStack(DataType::create_integer(4)),
        Command::PushStack(DataType::create_integer(5)),
        Command::Add,
        Command::PushStack(DataType::create_integer(3)),
        Command::Sub,
        Command::Halt(0),
    ];

    let mut frames = Vm::create_frames();

    let mut vm = Vm::new();

    let exit_code = vm.run(0, &commands, &mut frames).unwrap();

    println!("Exit Code: {}", exit_code);
    println!("{:?}", vm);
}

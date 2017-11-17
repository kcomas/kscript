
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::builder::command::{Command, DataHolder, DataType};
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};

pub fn create_builder<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    kscript.run_build_tokens_commands(program).unwrap();
    kscript
}

pub fn get_commands<T: Logger>(kscript: &Kscript<T>) -> &Vec<Command> {
    kscript.get_command_container().unwrap().get_commands()
}

pub fn last_is_clear(commands: &Vec<Command>) {
    assert_eq!(*commands.last().unwrap(), Command::ClearRegisters);
}

#[test]
fn var_assign_integer() {
    let kscript = create_builder("test = 1234", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("test".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1234)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn constant_assign_float() {
    let kscript = create_builder("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Const("TEST".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Float(1234.123)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

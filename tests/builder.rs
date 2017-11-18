
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

#[test]
fn var_assign_math() {
    let kscript = create_builder(
        "a = (1.234 * ((2 + 4.3) % 2) + 1 ^ 5)",
        VoidLogger::new((LoggerMode::Void)),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 17);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Float(1.234)))
    );
    assert_eq!(
        commands[2],
        Command::SetRegister(2, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(
        commands[3],
        Command::SetRegister(3, DataHolder::Anon(DataType::Float(4.3)))
    );
    assert_eq!(commands[4], Command::Addition(4, 2, 3));
    assert_eq!(commands[5], Command::SetRegister(5, DataHolder::Math(4)));
    assert_eq!(
        commands[6],
        Command::SetRegister(6, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(commands[7], Command::Modulus(7, 5, 6));
    assert_eq!(commands[8], Command::SetRegister(8, DataHolder::Math(7)));
    assert_eq!(
        commands[9],
        Command::SetRegister(9, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(
        commands[10],
        Command::SetRegister(10, DataHolder::Anon(DataType::Integer(5)))
    );
    assert_eq!(commands[11], Command::Exponent(11, 9, 10));
    assert_eq!(commands[12], Command::Multiply(12, 1, 8));
    assert_eq!(commands[13], Command::Addition(13, 12, 11));
    assert_eq!(commands[14], Command::SetRegister(14, DataHolder::Math(13)));
    assert_eq!(commands[15], Command::Assign(0, 14));
    last_is_clear(&commands);
}

#[test]
fn math_io_integer() {
    let kscript = create_builder("(2 * 3) > 1", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 7);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(3)))
    );
    assert_eq!(commands[2], Command::Multiply(2, 0, 1));
    assert_eq!(commands[3], Command::SetRegister(3, DataHolder::Math(2)));
    assert_eq!(
        commands[4],
        Command::SetRegister(4, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[5], Command::IoWrite(3, 4));
    last_is_clear(&commands);
}

#[test]
fn comment_op_comment() {
    let kscript = create_builder(
        "# this is a comment\n a = 1 # another comment",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_file() {
    let kscript = create_builder("myfile = 'hello'", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("myfile".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::File("hello".to_string())))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

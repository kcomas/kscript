
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::builder::command::{DataHolder, DataType};

pub fn create<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    kscript.run(program).unwrap();
    kscript
}

#[test]
fn var_assign_integer() {
    let kscript = create("test = 1234", VoidLogger::new(LoggerMode::Void));
    let v = kscript.get_root_scope().get_var("test").unwrap();
    assert_eq!(*v.borrow(), DataHolder::Anon(DataType::Integer(1234)));
}

#[test]
fn constant_assign_float() {
    let kscript = create("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));
    let v = kscript.get_root_scope().get_const("TEST").unwrap();
    assert_eq!(*v.borrow(), DataHolder::Anon(DataType::Float(1234.123)));
}

#[test]
fn var_assign_math() {
    let kscript = create(
        "a = (1.234 * ((2 + 4.3) % 2) + 1 ^ 5)",
        VoidLogger::new(LoggerMode::Void),
    );
    let v = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(
        *v.borrow(),
        DataHolder::Anon(DataType::Float(1.3701999999999996))
    );
}

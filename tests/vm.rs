
extern crate kscript;

use std::rc::Rc;
use std::cell::RefCell;
use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::builder::command::DataType;
use kscript::lang::vm::vm_types::{DataContainer, RefHolder};

fn create<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    kscript.run(program).unwrap();
    kscript
}

fn wrap(value: DataContainer) -> RefHolder {
    Rc::new(RefCell::new(value))
}

fn wrap_scalar(value: DataType) -> RefHolder {
    wrap(DataContainer::Scalar(value))
}

#[test]
fn var_assign_integer() {
    let kscript = create("test = 1234", VoidLogger::new(LoggerMode::Void));
    let v = kscript.get_root_scope().get_var("test").unwrap();
    assert_eq!(*v.borrow(), DataContainer::Scalar(DataType::Integer(1234)));
}

#[test]
fn constant_assign_float() {
    let kscript = create("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));
    let v = kscript.get_root_scope().get_const("TEST").unwrap();
    assert_eq!(
        *v.borrow(),
        DataContainer::Scalar(DataType::Float(1234.123))
    );
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
        DataContainer::Scalar(DataType::Float(1.3701999999999996))
    );
}

#[test]
fn comment_op_comment() {
    let kscript = create(
        "# this is a comment\n a = 1 # another comment",
        VoidLogger::new(LoggerMode::Void),
    );

    let v = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(*v.borrow(), DataContainer::Scalar(DataType::Integer(1)));
}

#[test]
fn var_assign_file() {
    let kscript = create("myfile = 'hello'", VoidLogger::new(LoggerMode::Void));

    let v = kscript.get_root_scope().get_var("myfile").unwrap();
    assert_eq!(
        *v.borrow(),
        DataContainer::Scalar(DataType::File("hello".to_string()))
    );
}

#[test]
fn var_assign_string() {
    let kscript = create("mystr = \"test # str\"", VoidLogger::new(LoggerMode::Void));

    let v = kscript.get_root_scope().get_var("mystr").unwrap();
    assert_eq!(
        *v.borrow(),
        DataContainer::Scalar(DataType::String("test # str".to_string()))
    );
}

#[test]
fn var_assign_array() {
    let kscript = create(
        "a = @[1, @[1.34, \"herp\"], (1 + 2 * 3), 1234]",
        VoidLogger::new(LoggerMode::Void),
    );

    let v = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(
        *v.borrow(),
        DataContainer::Vector(vec![
            wrap_scalar(DataType::Integer(1)),
            wrap(DataContainer::Vector(vec![
                wrap_scalar(DataType::Float(1.34)),
                wrap_scalar(DataType::String("herp".to_string())),
            ])),
            wrap_scalar(DataType::Integer(7)),
            wrap_scalar(DataType::Integer(1234)),
        ])
    );
}

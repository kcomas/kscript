
extern crate kscript;

use std::collections::HashMap;
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

#[test]
fn var_assign_dict() {
    let kscript = create(
        "d = %[\"asdf\": 1234, \"sub\": %[\"merp\": 3.45], \"arr\": @[1, 2, 4], \"herp\": \"derp\"]",
        VoidLogger::new(LoggerMode::Void),
    );

    let v = kscript.get_root_scope().get_var("d").unwrap();

    let mut map = HashMap::new();
    let mut sub_map = HashMap::new();

    sub_map.insert("merp".to_string(), wrap_scalar(DataType::Float(3.45)));

    map.insert("asdf".to_string(), wrap_scalar(DataType::Integer(1234)));
    map.insert("sub".to_string(), wrap(DataContainer::Hash(sub_map)));
    map.insert(
        "arr".to_string(),
        wrap(DataContainer::Vector(vec![
            wrap_scalar(DataType::Integer(1)),
            wrap_scalar(DataType::Integer(2)),
            wrap_scalar(DataType::Integer(4)),
        ])),
    );
    map.insert(
        "herp".to_string(),
        wrap_scalar(DataType::String("derp".to_string())),
    );

    assert_eq!(*v.borrow(), DataContainer::Hash(map));
}

#[test]
fn var_assign_bool_const_assign_bool() {
    let kscript = create("test = t; TESTD = f", VoidLogger::new(LoggerMode::Void));

    let t = kscript.get_root_scope().get_var("test").unwrap();
    let f = kscript.get_root_scope().get_const("TESTD").unwrap();

    assert_eq!(*t.borrow(), DataContainer::Scalar(DataType::Bool(true)));
    assert_eq!(*f.borrow(), DataContainer::Scalar(DataType::Bool(false)));
}

#[test]
fn vars_const_with_numbers() {
    let kscript = create(
        "py3 = 3; 23a = 3.12; 1S3 = 4",
        VoidLogger::new(LoggerMode::Void),
    );

    let py3 = kscript.get_root_scope().get_var("py3").unwrap();
    let a23 = kscript.get_root_scope().get_var("23a").unwrap();
    let s = kscript.get_root_scope().get_const("1S3").unwrap();

    assert_eq!(*py3.borrow(), DataContainer::Scalar(DataType::Integer(3)));
    assert_eq!(*a23.borrow(), DataContainer::Scalar(DataType::Float(3.12)));
    assert_eq!(*s.borrow(), DataContainer::Scalar(DataType::Integer(4)));
}

#[test]
fn nested_conditionial() {
    let kscript = create("a = ? ?1==1^? 3== 2", VoidLogger::new(LoggerMode::Void));

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(*a.borrow(), DataContainer::Scalar(DataType::Bool(true)));
}


extern crate kscript;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::builder::command::{Command, DataHolder, DataType};
use kscript::lang::vm::vm_types::{DataContainer, RefHolder, FunctionArg};

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
fn math_io_integer() {
    let kscript = create("(2 * 3) > 1", VoidLogger::new(LoggerMode::Void));
    let x = kscript.get_root_scope().get_var("x");
    assert!(x.is_none());
}

#[test]
fn math_from_access() {
    let kscript = create(
        "a = @[2, 5]; b = {|| 4}; c = %[\"t\": 2, \"g\": 1]; d = (a[1] + b|| + c[\"g\"]); d >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(
        *a.borrow(),
        DataContainer::Vector(vec![
            wrap(DataContainer::Scalar(DataType::Integer(2))),
            wrap(DataContainer::Scalar(DataType::Integer(5))),
        ])
    );

    let b = kscript.get_root_scope().get_var("b").unwrap();
    assert_eq!(
        *b.borrow(),
        DataContainer::Function(
            vec![],
            vec![
                Command::SetRegister(0, DataHolder::Anon(DataType::Integer(4))),
            ],
        )
    );

    let mut map = HashMap::new();
    map.insert(
        "t".to_string(),
        wrap(DataContainer::Scalar(DataType::Integer(2))),
    );
    map.insert(
        "g".to_string(),
        wrap(DataContainer::Scalar(DataType::Integer(1))),
    );

    let c = kscript.get_root_scope().get_var("c").unwrap();
    assert_eq!(*c.borrow(), DataContainer::Hash(map));

    let d = kscript.get_root_scope().get_var("d").unwrap();
    assert_eq!(*d.borrow(), DataContainer::Scalar(DataType::Integer(10)));
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
fn assign_conditional_true_false() {
    let kscript = create(
        "?1 == 2{a = 3}{b = \"test\"}",
        VoidLogger::new(LoggerMode::Void),
    );

    let a = kscript.get_root_scope().get_var("a");
    assert_eq!(a.is_none(), true);
    let b = kscript.get_root_scope().get_var("b").unwrap();
    assert_eq!(
        *b.borrow(),
        DataContainer::Scalar(DataType::String("test".to_string()))
    );
}

#[test]
fn nested_conditionial() {
    let kscript = create("a = ? ?1==1^? 3== 2", VoidLogger::new(LoggerMode::Void));

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(*a.borrow(), DataContainer::Scalar(DataType::Bool(true)));
}

#[test]
fn nested_conditionals_with_nested_data() {
    let kscript = create(
        "c=@[@[2]];a=??1=={|a|a}|1|&?2==c[0][0]",
        VoidLogger::new(LoggerMode::Void),
    );

    let c = kscript.get_root_scope().get_var("c").unwrap();
    assert_eq!(
        *c.borrow(),
        DataContainer::Vector(vec![
            wrap(DataContainer::Vector(
                vec![wrap_scalar(DataType::Integer(2))],
            )),
        ])
    );

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(*a.borrow(), DataContainer::Scalar(DataType::Bool(true)));
}

#[test]
fn function_in_dict() {
    let kscript = create(
        "d=%[\"test\":{|d|d=(d+1);d}][\"test\"]|2|",
        VoidLogger::new(LoggerMode::Void),
    );

    let d = kscript.get_root_scope().get_var("d").unwrap();
    assert_eq!(*d.borrow(), DataContainer::Scalar(DataType::Integer(3)));
}

#[test]
fn assign_loop_print() {
    let kscript = create(
        "a = 1; $a<5${a = (a + 1)} a > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(*a.borrow(), DataContainer::Scalar(DataType::Integer(5)));
}

#[test]
fn var_assign_var_function() {
    let kscript = create(
        "a = 1; b = {|a, &e, c| e = c; a }",
        VoidLogger::new(LoggerMode::Void),
    );

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(*a.borrow(), DataContainer::Scalar(DataType::Integer(1)));

    let b = kscript.get_root_scope().get_var("b").unwrap();
    assert_eq!(
        *b.borrow(),
        DataContainer::Function(
            vec![
                FunctionArg::Var("a".to_string()),
                FunctionArg::RefVar("e".to_string()),
                FunctionArg::Var("c".to_string()),
            ],
            vec![
                Command::SetRegister(0, DataHolder::Var("e".to_string())),
                Command::SetRegister(1, DataHolder::Var("c".to_string())),
                Command::Assign(0, 1),
                Command::ClearRegisters,
                Command::SetRegister(0, DataHolder::Var("a".to_string())),
            ],
        )
    );
}

#[test]
fn basic_function_call() {
    let kscript = create(
        "c = {|a| a > 1; 5}; d = c|\"test\"|",
        VoidLogger::new(LoggerMode::Void),
    );

    let c = kscript.get_root_scope().get_var("c").unwrap();
    assert_eq!(
        *c.borrow(),
        DataContainer::Function(
            vec![FunctionArg::Var("a".to_string())],
            vec![
                Command::SetRegister(0, DataHolder::Var("a".to_string())),
                Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1))),
                Command::IoWrite(0, 1),
                Command::ClearRegisters,
                Command::SetRegister(0, DataHolder::Anon(DataType::Integer(5))),
            ],
        )
    );

    let d = kscript.get_root_scope().get_var("d").unwrap();
    assert_eq!(*d.borrow(), DataContainer::Scalar(DataType::Integer(5)));
}

#[test]
fn anon_function_access_call() {
    let kscript = create(
        "{|&c| c||}|@[{|| \"test\"}, 12][0]| >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let c = kscript.get_root_scope().get_var("c");
    assert!(c.is_none());
}

#[test]
fn reassign_array_value() {
    let kscript = create(
        "a = @[1, \" \", 2]\n a[0] = \"test\"\n a >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(
        *a.borrow(),
        DataContainer::Vector(vec![
            wrap_scalar(DataType::String("test".to_string())),
            wrap_scalar(DataType::String(" ".to_string())),
            wrap_scalar(DataType::Integer(2)),
        ])
    );
}

#[test]
fn take_and_set_references() {
    let kscript = create(
        "a = @[1, 2, 3]; b =& a[1]; *b = 5; c = 3; b =& c; *b = 3.14",
        VoidLogger::new(LoggerMode::Void),
    );

    let a = kscript.get_root_scope().get_var("a").unwrap();
    assert_eq!(
        *a.borrow(),
        DataContainer::Vector(vec![
            wrap_scalar(DataType::Integer(1)),
            wrap_scalar(DataType::Integer(5)),
            wrap_scalar(DataType::Integer(3)),
        ])
    );

    let c = kscript.get_root_scope().get_var("c").unwrap();
    assert_eq!(*c.borrow(), DataContainer::Scalar(DataType::Float(3.14)));
}

#[test]
fn auto_deref_math() {
    let kscript = create(
        "a = 1; b =& a; c =& b; d = (a + b + c); e = @[10, 11][c]",
        VoidLogger::new(LoggerMode::Void),
    );

    let d = kscript.get_root_scope().get_var("d").unwrap();
    assert_eq!(*d.borrow(), DataContainer::Scalar(DataType::Integer(3)));

    let e = kscript.get_root_scope().get_var("e").unwrap();
    assert_eq!(*e.borrow(), DataContainer::Scalar(DataType::Integer(11)));
}

#[test]
fn add_underscores_to_vars() {
    let kscript = create("_a = 1; _1BSD = 2.21", VoidLogger::new(LoggerMode::Void));

    let _a = kscript.get_root_scope().get_var("_a").unwrap();
    assert_eq!(*_a.borrow(), DataContainer::Scalar(DataType::Integer(1)));

    let _1bsd = kscript.get_root_scope().get_const("_1BSD").unwrap();
    assert_eq!(
        *_1bsd.borrow(),
        DataContainer::Scalar(DataType::Float(2.21))
    );
}

#[test]
fn casting_operations() {
    let kscript = create(
        "str = \"3.14\"; float = `p str; s2 = `s float; bool = `b s2",
        VoidLogger::new(LoggerMode::Void),
    );

    let s = kscript.get_root_scope().get_var("str").unwrap();
    assert_eq!(
        *s.borrow(),
        DataContainer::Scalar(DataType::String("3.14".to_string()))
    );

    let float = kscript.get_root_scope().get_var("float").unwrap();
    assert_eq!(
        *float.borrow(),
        DataContainer::Scalar(DataType::Float(3.14))
    );

    let s2 = kscript.get_root_scope().get_var("s2").unwrap();
    assert_eq!(
        *s2.borrow(),
        DataContainer::Scalar(DataType::String("3.14".to_string()))
    );

    let b = kscript.get_root_scope().get_var("bool").unwrap();
    assert_eq!(*b.borrow(), DataContainer::Scalar(DataType::Bool(true)));
}

#[test]
fn array_length() {
    let kscript = create(
        "a = @[0, 1, 2, 3, 4]; b = @? a",
        VoidLogger::new(LoggerMode::Void),
    );

    let b = kscript.get_root_scope().get_var("b").unwrap();
    assert_eq!(*b.borrow(), DataContainer::Scalar(DataType::Integer(5)));
}

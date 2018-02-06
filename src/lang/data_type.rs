use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct FunctionPointer {
    command_index: usize,
    args: usize,
    length: usize,
}

#[derive(Debug)]
pub enum DataType {
    Bool(Rc<RefCell<bool>>),
    Integer(Rc<RefCell<i64>>),
    Float(Rc<RefCell<f64>>),
    String(Rc<RefCell<String>>),
    Function(Rc<RefCell<FunctionPointer>>),
}

impl DataType {
    pub fn shallow_clone(&self) -> DataType {
        match *self {
            DataType::Bool(ref b) => DataType::Bool(Rc::clone(b)),
            DataType::Integer(ref int) => DataType::Integer(Rc::clone(int)),
            DataType::Float(ref float) => DataType::Float(Rc::clone(float)),
            DataType::String(ref string) => DataType::String(Rc::clone(string)),
            DataType::Function(ref function) => DataType::Function(Rc::clone(function)),
        }
    }

    pub fn create_bool(b: bool) -> DataType {
        return DataType::Bool(Rc::new(RefCell::new(b)));
    }

    pub fn create_integer(int: i64) -> DataType {
        return DataType::Integer(Rc::new(RefCell::new(int)));
    }
}

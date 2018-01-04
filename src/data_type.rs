use std::rc::Rc;
use std::cell::RefCell;
use super::command::SharedCommands;
use super::error::RuntimeError;

pub type SharedArray = Rc<RefCell<Vec<DataType>>>;

#[derive(Debug)]
pub enum DataType {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Rc<RefCell<String>>),
    Array(SharedArray),
    // array ref, index
    ArrayAccess(SharedArray, usize),
    // commands ref, num args
    Function(SharedCommands, usize),
}

impl DataType {
    pub fn is_fuction(&self) -> bool {
        if let DataType::Function(_, _) = *self {
            return true;
        }
        false
    }

    pub fn get_function(&self) -> Result<(SharedCommands, usize), RuntimeError> {
        if let DataType::Function(ref commands, num_args) = *self {
            return Ok((Rc::clone(commands), num_args));
        }
        Err(RuntimeError::NotAFunction(self.clone()))
    }
}

impl Clone for DataType {
    fn clone(&self) -> DataType {
        match *self {
            DataType::Bool(b) => DataType::Bool(b),
            DataType::Integer(int) => DataType::Integer(int),
            DataType::Float(float) => DataType::Float(float),
            DataType::String(ref string) => DataType::String(Rc::clone(string)),
            DataType::Array(ref array) => DataType::Array(Rc::clone(array)),
            DataType::ArrayAccess(ref array, index) => {
                DataType::ArrayAccess(Rc::clone(array), index)
            }
            DataType::Function(ref commands, index) => {
                DataType::Function(Rc::clone(commands), index)
            }
        }
    }
}

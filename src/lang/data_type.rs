use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Sub};
use super::error::RuntimeError;

#[derive(Debug)]
pub struct FunctionPointer {
    pub command_index: usize,
    pub num_arguments: usize,
    pub num_locals: usize,
    pub length: usize,
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
    pub fn is_int(&self) -> bool {
        if let DataType::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            DataType::Bool(ref b) => if *b.borrow() {
                1
            } else {
                0
            },
            DataType::Integer(ref int) => *int.borrow(),
            DataType::Float(ref float) => *float.borrow() as i64,
            _ => 0,
        }
    }

    pub fn is_float(&self) -> bool {
        if let DataType::Float(_) = *self {
            return true;
        }
        false
    }

    pub fn as_float(&self) -> f64 {
        match *self {
            DataType::Bool(ref b) => if *b.borrow() {
                1.0
            } else {
                0.0
            },
            DataType::Integer(ref int) => *int.borrow() as f64,
            DataType::Float(ref float) => *float.borrow(),
            _ => 0.0,
        }
    }

    pub fn get_function(&self) -> Result<&Rc<RefCell<FunctionPointer>>, RuntimeError> {
        if let DataType::Function(ref function) = *self {
            return Ok(function);
        }
        Err(RuntimeError::TargetNotAFunction)
    }

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

    pub fn create_float(float: f64) -> DataType {
        return DataType::Float(Rc::new(RefCell::new(float)));
    }

    pub fn create_string(string: String) -> DataType {
        return DataType::String(Rc::new(RefCell::new(string)));
    }

    pub fn create_function(function: FunctionPointer) -> DataType {
        return DataType::Function(Rc::new(RefCell::new(function)));
    }
}

impl Add for DataType {
    type Output = DataType;

    fn add(self, right: DataType) -> DataType {
        if self.is_float() || right.is_float() {
            return DataType::create_float(self.as_float() + right.as_float());
        }
        return DataType::create_integer(self.as_int() + right.as_int());
    }
}

impl Sub for DataType {
    type Output = DataType;

    fn sub(self, right: DataType) -> DataType {
        if self.is_float() || right.is_float() {
            return DataType::create_float(self.as_float() - right.as_float());
        }
        return DataType::create_integer(self.as_int() - right.as_int());
    }
}

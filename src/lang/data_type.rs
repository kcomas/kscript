use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::fmt;
use super::command::SharedCommands;
use super::error::RuntimeError;

pub type SharedString = Rc<RefCell<String>>;
pub type SharedArray = Rc<RefCell<Vec<DataType>>>;

#[derive(Debug)]
pub enum DataType {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(SharedString),
    Array(SharedArray),
    // commands ref, num args
    Function(SharedCommands, usize),
}

impl DataType {
    pub fn is_bool(&self) -> bool {
        if let DataType::Bool(_) = *self {
            return true;
        }
        false
    }

    pub fn as_bool(&self) -> bool {
        match *self {
            DataType::Bool(b) => b,
            DataType::Integer(int) => int != 0,
            DataType::Float(float) => float != 0.0,
            _ => false,
        }
    }

    pub fn get_bool(&self) -> Result<bool, RuntimeError> {
        if let DataType::Bool(b) = *self {
            return Ok(b);
        }
        Err(RuntimeError::NotABool(self.clone()))
    }

    pub fn is_int(&self) -> bool {
        if let DataType::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            DataType::Bool(b) => if b {
                1
            } else {
                0
            },
            DataType::Integer(int) => int,
            DataType::Float(float) => float as i64,
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
            DataType::Bool(b) => if b {
                1.0
            } else {
                0.0
            },
            DataType::Integer(int) => int as f64,
            DataType::Float(float) => float,
            _ => 0.0,
        }
    }

    pub fn is_string(&self) -> bool {
        if let DataType::String(_) = *self {
            return true;
        }
        false
    }

    pub fn as_string(&self) -> SharedString {
        match *self {
            DataType::String(ref string) => Rc::clone(string),
            _ => Rc::new(RefCell::new(String::new())),
        }
    }

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
            DataType::Function(ref commands, index) => {
                DataType::Function(Rc::clone(commands), index)
            }
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::Bool(b) => write!(f, "{}", if b { "t" } else { "f" }),
            DataType::Integer(num) => write!(f, "{}", num),
            DataType::Float(float) => write!(f, "{}", float),
            DataType::String(ref string) => write!(f, "{}", string.borrow()),
            _ => write!(f, "NYI"),
        }
    }
}

impl Add for DataType {
    type Output = DataType;

    fn add(self, right: DataType) -> DataType {
        if self.is_string() && right.is_string() {
            let left = self.as_string();
            let left = left.borrow().clone();
            let right = right.as_string();
            let right = right.borrow().clone();
            return DataType::String(Rc::new(RefCell::new(left + &right)));
        } else if self.is_float() || right.is_float() {
            return DataType::Float(self.as_float() + right.as_float());
        }
        DataType::Integer(self.as_int() + right.as_int())
    }
}

impl Sub for DataType {
    type Output = DataType;

    fn sub(self, right: DataType) -> DataType {
        if self.is_float() || right.is_float() {
            return DataType::Float(self.as_float() - right.as_float());
        }
        DataType::Integer(self.as_int() - right.as_int())
    }
}

impl Mul for DataType {
    type Output = DataType;

    fn mul(self, right: DataType) -> DataType {
        if self.is_string() && right.is_int() {
            let left = self.as_string();
            let left = left.borrow();
            let right = right.as_int();
            return DataType::String(Rc::new(RefCell::new(left.repeat(right as usize))));
        } else if self.is_float() || right.is_float() {
            return DataType::Float(self.as_float() * right.as_float());
        }
        DataType::Integer(self.as_int() * right.as_int())
    }
}

impl Div for DataType {
    type Output = DataType;

    fn div(self, right: DataType) -> DataType {
        if self.is_float() || right.is_float() {
            return DataType::Float(self.as_float() / right.as_float());
        }
        DataType::Integer(self.as_int() / right.as_int())
    }
}

use std::ops::{Add, Sub};
use super::function::FunctionPointer;
use super::error::RuntimeError;
use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<MemoryAddress>),
    Function(FunctionPointer),
}

impl DataHolder {
    pub fn get_bool(&self) -> Result<bool, RuntimeError> {
        if let DataHolder::Bool(b) = *self {
            return Ok(b);
        }
        Err(RuntimeError::TargetNotABool)
    }

    pub fn is_int(&self) -> bool {
        if let DataHolder::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            DataHolder::Bool(b) => if b {
                1
            } else {
                0
            },
            DataHolder::Integer(int) => int,
            DataHolder::Float(float) => float as i64,
            _ => 0,
        }
    }

    pub fn is_float(&self) -> bool {
        if let DataHolder::Float(_) = *self {
            return true;
        }
        false
    }

    pub fn as_float(&self) -> f64 {
        match *self {
            DataHolder::Bool(b) => if b {
                1.0
            } else {
                0.0
            },
            DataHolder::Integer(int) => int as f64,
            DataHolder::Float(float) => float,
            _ => 0.0,
        }
    }

    pub fn get_function(&self) -> Result<&FunctionPointer, RuntimeError> {
        if let DataHolder::Function(ref pointer) = *self {
            return Ok(pointer);
        }
        Err(RuntimeError::InvalidFunction)
    }
}

impl Add for DataHolder {
    type Output = DataHolder;

    fn add(self, right: DataHolder) -> DataHolder {
        if self.is_float() || right.is_float() {
            return DataHolder::Float(self.as_float() + right.as_float());
        }
        DataHolder::Integer(self.as_int() + right.as_int())
    }
}

impl Sub for DataHolder {
    type Output = DataHolder;

    fn sub(self, right: DataHolder) -> DataHolder {
        if self.is_float() || right.is_float() {
            return DataHolder::Float(self.as_float() - right.as_float());
        }
        DataHolder::Integer(self.as_int() - right.as_int())
    }
}

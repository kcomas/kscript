use super::function::FunctionPointer;
use std::ops::{Add, Sub};

pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Function(FunctionPointer),
}

pub enum RefHolder<'a> {
    Bool(&'a bool),
    Integer(&'a i64),
    Float(&'a f64),
    Function(&'a FunctionPointer),
}

impl<'a> RefHolder<'a> {
    pub fn is_int(&self) -> bool {
        if let RefHolder::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            RefHolder::Bool(b) => if *b {
                1
            } else {
                0
            },
            RefHolder::Integer(int) => *int,
            RefHolder::Float(float) => *float as i64,
            _ => 0,
        }
    }

    pub fn is_float(&self) -> bool {
        if let RefHolder::Float(_) = *self {
            return true;
        }
        false
    }

    pub fn as_float(&self) -> f64 {
        match *self {
            RefHolder::Bool(b) => if *b {
                1.0
            } else {
                0.0
            },
            RefHolder::Integer(int) => *int as f64,
            RefHolder::Float(float) => *float,
            _ => 0.0,
        }
    }
}

impl<'a> Add for RefHolder<'a> {
    type Output = DataHolder;

    fn add(self, right: RefHolder<'a>) -> DataHolder {
        if self.is_float() || right.is_float() {
            return DataHolder::Float(self.as_float() + right.as_float());
        }
        DataHolder::Integer(self.as_int() + right.as_int())
    }
}

impl<'a> Sub for RefHolder<'a> {
    type Output = DataHolder;

    fn sub(self, right: RefHolder<'a>) -> DataHolder {
        if self.is_float() || right.is_float() {
            return DataHolder::Float(self.as_float() - right.as_float());
        }
        DataHolder::Integer(self.as_int() - right.as_int())
    }
}

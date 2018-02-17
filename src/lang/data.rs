use std::ops::{Add, Sub};
use super::function::FunctionPointer;

#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    Function(FunctionPointer),
    String(String),
}

#[derive(Debug)]
pub enum RefHolder<'a> {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    Function(FunctionPointer),
    String(&'a String),
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
            RefHolder::Bool(b) => if b {
                1
            } else {
                0
            },
            RefHolder::Integer(int) => int,
            RefHolder::Float(float) => float as i64,
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
            RefHolder::Bool(b) => if b {
                1.0
            } else {
                0.0
            },
            RefHolder::Integer(int) => int as f64,
            RefHolder::Float(float) => float,
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

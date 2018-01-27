use std::ops::Add;
use super::memory::Function;

#[derive(Debug)]
pub enum RefDataHolder<'a> {
    Bool(&'a bool),
    Integer(&'a i64),
    Float(&'a f64),
    Function(&'a Function),
}

impl<'a> RefDataHolder<'a> {
    pub fn is_int(&self) -> bool {
        if let RefDataHolder::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            RefDataHolder::Bool(b) => if *b {
                1
            } else {
                0
            },
            RefDataHolder::Integer(int) => *int,
            RefDataHolder::Float(float) => *float as i64,
            _ => 0,
        }
    }

    pub fn is_float(&self) -> bool {
        if let RefDataHolder::Float(_) = *self {
            return true;
        }
        false
    }

    pub fn as_float(&self) -> f64 {
        match *self {
            RefDataHolder::Bool(b) => if *b {
                1.0
            } else {
                0.0
            },
            RefDataHolder::Integer(int) => *int as f64,
            RefDataHolder::Float(float) => *float,
            _ => 0.0,
        }
    }
}

impl<'a> Add for RefDataHolder<'a> {
    type Output = DataHolder;

    fn add(self, right: RefDataHolder<'a>) -> DataHolder {
        if self.is_float() || right.is_float() {
            return DataHolder::Float(self.as_float() + right.as_float());
        }
        DataHolder::Integer(self.as_int() + right.as_int())
    }
}

#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Function(Function),
}

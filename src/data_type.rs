use std::ops::{Add, Sub};
use std::fmt;

#[derive(Debug)]
pub enum DataType {
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl DataType {
    pub fn is_int(&self) -> bool {
        if let DataType::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn get_int(&self) -> i64 {
        match *self {
            DataType::Bool(b) => match b {
                true => 1,
                false => 0,
            },
            DataType::Integer(int) => int,
            DataType::Float(float) => float as i64,
        }
    }

    pub fn is_float(&self) -> bool {
        if let DataType::Float(_) = *self {
            return true;
        }
        false
    }

    pub fn get_float(&self) -> f64 {
        match *self {
            DataType::Bool(b) => match b {
                true => 1.0,
                false => 0.0,
            },
            DataType::Integer(int) => int as f64,
            DataType::Float(float) => float,
        }
    }

    pub fn get_bool(&self) -> bool {
        match *self {
            DataType::Bool(b) => b,
            DataType::Integer(int) => int != 0,
            DataType::Float(float) => float != 0.0,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::Bool(b) => write!(f, "{}", b),
            DataType::Integer(int) => write!(f, "{}", int),
            DataType::Float(float) => write!(f, "{}", float),
        }
    }
}

impl Clone for DataType {
    fn clone(&self) -> DataType {
        match *self {
            DataType::Bool(b) => DataType::Bool(b),
            DataType::Integer(int) => DataType::Integer(int),
            DataType::Float(float) => DataType::Float(float),
        }
    }
}

impl Add for DataType {
    type Output = DataType;

    fn add(self, right: DataType) -> DataType {
        if self.is_float() || right.is_float() {
            return DataType::Float(self.get_float() + right.get_float());
        }
        DataType::Integer(self.get_int() + right.get_int())
    }
}

impl Sub for DataType {
    type Output = DataType;

    fn sub(self, right: DataType) -> DataType {
        if self.is_float() || right.is_float() {
            return DataType::Float(self.get_float() - right.get_float());
        }
        DataType::Integer(self.get_int() - right.get_int())
    }
}

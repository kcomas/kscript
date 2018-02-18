use std::ops::{Add, Sub};
use super::data::Data;
use super::error::RuntimeError;

#[derive(Debug, Clone)]
pub enum MemoryItem {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(usize),
    Function(usize),
}

impl MemoryItem {
    pub fn get_bool(&self) -> Result<bool, RuntimeError> {
        if let MemoryItem::Bool(b) = *self {
            return Ok(b);
        }
        Err(RuntimeError::TargetIsNotABool)
    }

    pub fn is_int(&self) -> bool {
        if let MemoryItem::Integer(_) = *self {
            return true;
        }
        false
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            MemoryItem::Bool(b) => if b {
                1
            } else {
                0
            },
            MemoryItem::Integer(int) => int,
            MemoryItem::Float(float) => float as i64,
            _ => 0,
        }
    }

    pub fn is_float(&self) -> bool {
        if let MemoryItem::Float(_) = *self {
            return true;
        }
        false
    }

    pub fn as_float(&self) -> f64 {
        match *self {
            MemoryItem::Bool(b) => if b {
                1.0
            } else {
                0.0
            },
            MemoryItem::Integer(int) => int as f64,
            MemoryItem::Float(float) => float,
            _ => 0.0,
        }
    }

    pub fn is_function(&self) -> bool {
        if let MemoryItem::Function(_) = *self {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub enum MemoryAddress {
    Counted(MemoryItem),
    Fixed(MemoryItem),
}

impl MemoryAddress {
    pub fn get_item(&self) -> &MemoryItem {
        match *self {
            MemoryAddress::Counted(ref item) | MemoryAddress::Fixed(ref item) => item,
        }
    }

    pub fn get_bool(&self) -> Result<bool, RuntimeError> {
        self.get_item().get_bool()
    }

    pub fn is_int(&self) -> bool {
        self.get_item().is_int()
    }

    pub fn as_int(&self) -> i64 {
        self.get_item().as_int()
    }

    pub fn is_float(&self) -> bool {
        self.get_item().is_float()
    }

    pub fn as_float(&self) -> f64 {
        self.get_item().as_float()
    }

    pub fn is_function(&self) -> bool {
        self.get_item().is_function()
    }
}

impl<'a> Add for &'a MemoryAddress {
    type Output = Data;

    fn add(self, right: &'a MemoryAddress) -> Data {
        if self.is_float() || right.is_float() {
            return Data::Float(self.as_float() + self.as_float());
        }
        Data::Integer(self.as_int() + right.as_int())
    }
}

impl<'a> Sub for &'a MemoryAddress {
    type Output = Data;

    fn sub(self, right: &'a MemoryAddress) -> Data {
        if self.is_float() || right.is_float() {
            return Data::Float(self.as_float() - self.as_float());
        }
        Data::Integer(self.as_int() - right.as_int())
    }
}

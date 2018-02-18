use super::function::FunctionPointer;
use super::error::RuntimeError;

#[derive(Debug, Clone)]
pub enum MemoryItem {
    Bool(bool),
    Integer(i64),
    Float(f64),
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

    pub fn is_float(&self) -> bool {
        if let MemoryItem::Float(_) = *self {
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

    pub fn is_float(&self) -> bool {
        self.get_item().is_float()
    }
}

use std::collections::VecDeque;
use super::error::RuntimeError;
use super::function::FunctionPointer;
use super::data::{DataHolder, RefHolder};

#[derive(Debug)]
pub struct Counted<T> {
    value: T,
    ref_count: usize,
}

impl<T> Counted<T> {
    pub fn new(value: T) -> Counted<T> {
        Counted {
            value: value,
            ref_count: 1,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn inc(&mut self) -> usize {
        self.ref_count += 1;
        self.ref_count
    }

    pub fn dec(&mut self) -> usize {
        self.ref_count -= 1;
        self.ref_count
    }
}

#[derive(Debug)]
pub struct Container<T> {
    dynamic: Vec<Counted<T>>,
    free: VecDeque<usize>,
    fixed: Vec<T>,
}

impl<T> Container<T> {
    pub fn new() -> Container<T> {
        Container {
            dynamic: Vec::new(),
            free: VecDeque::new(),
            fixed: Vec::new(),
        }
    }

    pub fn get_dynamic(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.dynamic.get(index) {
            return Ok(item.get());
        }
        Err(RuntimeError::InvalidMemoryAccess)
    }

    pub fn get_fixed(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.fixed.get(index) {
            return Ok(item);
        }
        Err(RuntimeError::InvalidMemoryAccess)
    }

    pub fn inc(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.dynamic.get_mut(index) {
            return Ok(item.inc());
        }
        Err(RuntimeError::InvalidRefInc)
    }

    pub fn dec(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.dynamic.get_mut(index) {
            let count = item.dec();
            if count == 0 {
                self.free.push_back(index);
            }
            return Ok(count);
        }
        Err(RuntimeError::InvalidRefDec)
    }

    pub fn insert_dynamic(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop_front() {
            self.dynamic[index] = Counted::new(value);
            return index;
        }
        self.dynamic.push(Counted::new(value));
        self.dynamic.len() - 1
    }

    pub fn insert_fixed(&mut self, value: T) -> usize {
        self.fixed.push(value);
        self.fixed.len() - 1
    }
}

#[derive(Debug)]
pub enum MemoryItem {
    Bool(usize),
    Integer(usize),
    Float(usize),
    Function(usize),
}

#[derive(Debug)]
pub enum MemoryAddress {
    Dynamic(MemoryItem),
    Fixed(MemoryItem),
}

#[derive(Debug)]
pub struct Memory {
    bools: Container<bool>,
    integers: Container<i64>,
    floats: Container<f64>,
    functions: Container<FunctionPointer>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bools: Container::new(),
            integers: Container::new(),
            floats: Container::new(),
            functions: Container::new(),
        }
    }

    pub fn get(&self, address: &MemoryAddress) -> Result<RefHolder, RuntimeError> {
        let holder = match *address {
            MemoryAddress::Dynamic(ref item) => match *item {
                MemoryItem::Bool(index) => RefHolder::Bool(self.bools.get_dynamic(index)?),
                MemoryItem::Integer(index) => RefHolder::Integer(self.integers.get_dynamic(index)?),
                MemoryItem::Float(index) => RefHolder::Float(self.floats.get_dynamic(index)?),
                MemoryItem::Function(index) => {
                    RefHolder::Function(self.functions.get_dynamic(index)?)
                }
            },
            MemoryAddress::Fixed(ref item) => match *item {
                MemoryItem::Bool(index) => RefHolder::Bool(self.bools.get_fixed(index)?),
                MemoryItem::Integer(index) => RefHolder::Integer(self.integers.get_fixed(index)?),
                MemoryItem::Float(index) => RefHolder::Float(self.floats.get_fixed(index)?),
                MemoryItem::Function(index) => {
                    RefHolder::Function(self.functions.get_fixed(index)?)
                }
            },
        };
        Ok(holder)
    }

    fn inc(&mut self, address: &MemoryAddress) -> Result<usize, RuntimeError> {
        let count = match *address {
            MemoryAddress::Dynamic(ref item) => match *item {
                MemoryItem::Bool(index) => self.bools.inc(index)?,
                MemoryItem::Integer(index) => self.integers.inc(index)?,
                MemoryItem::Float(index) => self.floats.inc(index)?,
                MemoryItem::Function(index) => self.functions.inc(index)?,
            },
            MemoryAddress::Fixed(_) => 1,
        };
        Ok(count)
    }

    fn dec(&mut self, address: &MemoryAddress) -> Result<usize, RuntimeError> {
        let count = match *address {
            MemoryAddress::Dynamic(ref item) => match *item {
                MemoryItem::Bool(index) => self.bools.dec(index)?,
                MemoryItem::Integer(index) => self.integers.dec(index)?,
                MemoryItem::Float(index) => self.floats.dec(index)?,
                MemoryItem::Function(index) => self.functions.dec(index)?,
            },
            MemoryAddress::Fixed(_) => 1,
        };
        Ok(count)
    }
}

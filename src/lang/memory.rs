use std::collections::VecDeque;
use super::function::FunctionPointer;
use super::address::{MemoryAddress, MemoryItem};
use super::data::{Data, RefData};
use super::error::RuntimeError;

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
pub struct Collection<T> {
    counted: Vec<Counted<T>>,
    free: VecDeque<usize>,
    fixed: Vec<T>,
}

impl<T> Collection<T> {
    pub fn new() -> Collection<T> {
        Collection {
            counted: Vec::new(),
            free: VecDeque::new(),
            fixed: Vec::new(),
        }
    }

    pub fn get_counted(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.counted.get(index) {
            return Ok(item.get());
        }
        Err(RuntimeError::CannotLoadCountedMemory)
    }

    pub fn get_fixed(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.fixed.get(index) {
            return Ok(item);
        }
        Err(RuntimeError::CannotLoadFixedMemory)
    }

    pub fn inc(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.counted.get_mut(index) {
            return Ok(item.inc());
        }
        Err(RuntimeError::CannotIncRefCount)
    }

    pub fn dec(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.counted.get_mut(index) {
            let count = item.dec();
            if count == 0 {
                self.free.push_back(index);
            }
            return Ok(count);
        }
        Err(RuntimeError::CannotDecRefCount)
    }

    pub fn insert_counted(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop_front() {
            self.counted[index] = Counted::new(value);
            return index;
        }
        self.counted.push(Counted::new(value));
        self.counted.len() - 1
    }

    pub fn insert_fixed(&mut self, value: T) -> usize {
        self.fixed.push(value);
        self.fixed.len() - 1
    }
}

#[derive(Debug)]
pub struct Memory {
    strings: Collection<String>,
    functions: Collection<FunctionPointer>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            strings: Collection::new(),
            functions: Collection::new(),
        }
    }

    pub fn get(&self, address: &MemoryAddress) -> Result<Option<RefData>, RuntimeError> {
        let item = match *address {
            MemoryAddress::Counted(ref item) => match *item {
                MemoryItem::String(index) => {
                    Some(RefData::String(self.strings.get_counted(index)?))
                }
                MemoryItem::Function(index) => {
                    Some(RefData::Function(self.functions.get_counted(index)?))
                }
                _ => None,
            },
            MemoryAddress::Fixed(ref item) => match *item {
                MemoryItem::String(index) => Some(RefData::String(self.strings.get_fixed(index)?)),
                MemoryItem::Function(index) => {
                    Some(RefData::Function(self.functions.get_fixed(index)?))
                }
                _ => None,
            },
        };
        Ok(item)
    }

    pub fn inc(&mut self, address: &MemoryAddress) -> Result<usize, RuntimeError> {
        let count = match *address {
            MemoryAddress::Counted(ref item) => match *item {
                MemoryItem::Function(index) => self.functions.inc(index)?,
                _ => 1,
            },
            MemoryAddress::Fixed(_) => 1,
        };
        Ok(count)
    }

    pub fn dec(&mut self, address: &MemoryAddress) -> Result<usize, RuntimeError> {
        let count = match *address {
            MemoryAddress::Counted(ref item) => match *item {
                MemoryItem::Function(index) => self.functions.dec(index)?,
                _ => 1,
            },
            MemoryAddress::Fixed(_) => 1,
        };
        Ok(count)
    }

    pub fn insert_counted(&mut self, data: Data) -> MemoryAddress {
        let item = match data {
            Data::Bool(b) => MemoryItem::Bool(b),
            Data::Integer(int) => MemoryItem::Integer(int),
            Data::Float(float) => MemoryItem::Float(float),
            Data::String(string) => MemoryItem::String(self.strings.insert_counted(string)),
            Data::Function(pointer) => MemoryItem::Function(self.functions.insert_counted(pointer)),
        };
        MemoryAddress::Counted(item)
    }

    pub fn insert_fixed(&mut self, data: Data) -> MemoryAddress {
        let item = match data {
            Data::Bool(b) => MemoryItem::Bool(b),
            Data::Integer(int) => MemoryItem::Integer(int),
            Data::Float(float) => MemoryItem::Float(float),
            Data::String(string) => MemoryItem::String(self.strings.insert_fixed(string)),
            Data::Function(pointer) => MemoryItem::Function(self.functions.insert_fixed(pointer)),
        };
        MemoryAddress::Fixed(item)
    }
}

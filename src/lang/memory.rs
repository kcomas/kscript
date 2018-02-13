use std::collections::VecDeque;
use super::data::{DataHolder, RefHolder};
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
            ref_count: 0,
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
    counted: Vec<Counted<T>>,
    free: VecDeque<usize>,
    fixed: Vec<T>,
}

impl<T> Container<T> {
    pub fn new() -> Container<T> {
        Container {
            counted: Vec::new(),
            free: VecDeque::new(),
            fixed: Vec::new(),
        }
    }

    pub fn get_counted(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.counted.get(index) {
            return Ok(item.get());
        }
        Err(RuntimeError::CannotGetCountedMemoryItem)
    }

    pub fn get_fixed(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.fixed.get(index) {
            return Ok(item);
        }
        Err(RuntimeError::CannotGetFixedMemoryItem)
    }

    pub fn inc(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.counted.get_mut(index) {
            return Ok(item.inc());
        }
        Err(RuntimeError::CannotIncRef)
    }

    pub fn dec(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.counted.get_mut(index) {
            let count = item.dec();
            if count == 0 {
                self.free.push_back(index);
            }
            return Ok(count);
        }
        Err(RuntimeError::CannotDecRef)
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
pub enum MemoryItem {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(usize),
}

#[derive(Debug)]
pub enum MemoryAddress {
    Counted(MemoryItem),
    Fixed(MemoryItem),
}

#[derive(Debug)]
pub struct Memory {
    strings: Container<String>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            strings: Container::new(),
        }
    }

    pub fn get(&self, address: &MemoryAddress) -> Result<RefHolder, RuntimeError> {
        let item = match *address {
            MemoryAddress::Counted(ref item) => match *item {
                MemoryItem::Bool(b) => RefHolder::Bool(b),
                MemoryItem::Integer(int) => RefHolder::Integer(int),
                MemoryItem::Float(float) => RefHolder::Float(float),
                MemoryItem::Char(c) => RefHolder::Char(c),
                MemoryItem::String(index) => RefHolder::String(self.strings.get_counted(index)?),
            },
            MemoryAddress::Fixed(ref item) => match *item {
                MemoryItem::Bool(b) => RefHolder::Bool(b),
                MemoryItem::Integer(int) => RefHolder::Integer(int),
                MemoryItem::Float(float) => RefHolder::Float(float),
                MemoryItem::Char(c) => RefHolder::Char(c),
                MemoryItem::String(index) => RefHolder::String(self.strings.get_fixed(index)?),
            },
        };
        Ok(item)
    }

    pub fn inc(&mut self, address: &MemoryAddress) -> Result<usize, RuntimeError> {
        let count = match *address {
            MemoryAddress::Counted(ref item) => match *item {
                MemoryItem::String(index) => self.strings.inc(index)?,
                _ => 1,
            },
            MemoryAddress::Fixed(_) => 1,
        };
        Ok(count)
    }

    pub fn dec(&mut self, address: &MemoryAddress) -> Result<usize, RuntimeError> {
        let count = match *address {
            MemoryAddress::Counted(ref item) => match *item {
                MemoryItem::String(index) => self.strings.dec(index)?,
                _ => 1,
            },
            MemoryAddress::Fixed(_) => 1,
        };
        Ok(count)
    }
}

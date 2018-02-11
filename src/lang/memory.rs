use std::rc::Rc;
use std::cell::RefCell;
use super::function::FunctionPointer;
use super::error::RuntimeError;
use super::data::DataHolder;

#[derive(Debug)]
pub struct MemoryContainer<T: Clone> {
    stack: Vec<T>,
    fixed: Vec<T>,
}

impl<T> MemoryContainer<T>
where
    T: Clone,
{
    pub fn new() -> MemoryContainer<T> {
        MemoryContainer {
            stack: Vec::new(),
            fixed: Vec::new(),
        }
    }

    pub fn pop_stack(&mut self, index: usize) -> Result<T, RuntimeError> {
        if index != self.stack.len() - 1 {
            return Err(RuntimeError::CannotPopMemoryStack);
        }
        if let Some(item) = self.stack.pop() {
            return Ok(item);
        }
        Err(RuntimeError::CannotPopMemoryStack)
    }

    pub fn get_stack(&self, index: usize) -> Result<T, RuntimeError> {
        if let Some(item) = self.stack.get(index) {
            return Ok(item.clone());
        }
        Err(RuntimeError::CannotLoadMemoryStackItem)
    }

    pub fn get_fixed(&self, index: usize) -> Result<T, RuntimeError> {
        if let Some(item) = self.fixed.get(index) {
            return Ok(item.clone());
        }
        Err(RuntimeError::CannotLoadMemoryFixedItem)
    }

    pub fn insert_stack(&mut self, value: T) -> usize {
        self.stack.push(value);
        self.stack.len() - 1
    }

    pub fn insert_fixed(&mut self, value: T) -> usize {
        self.fixed.push(value);
        self.stack.len() - 1
    }
}

#[derive(Debug)]
pub enum MemoryItem {
    Bool(usize),
    Integer(usize),
    Float(usize),
    String(usize),
    Array(usize),
    Function(usize),
}

#[derive(Debug)]
pub enum MemoryAddress {
    Stack(MemoryItem),
    Fixed(MemoryItem),
}

#[derive(Debug)]
pub struct Memory {
    bools: MemoryContainer<bool>,
    integers: MemoryContainer<i64>,
    floats: MemoryContainer<f64>,
    strings: MemoryContainer<Rc<RefCell<String>>>,
    arrays: MemoryContainer<Vec<Rc<RefCell<DataHolder>>>>,
    functions: MemoryContainer<FunctionPointer>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bools: MemoryContainer::new(),
            integers: MemoryContainer::new(),
            floats: MemoryContainer::new(),
            strings: MemoryContainer::new(),
            arrays: MemoryContainer::new(),
            functions: MemoryContainer::new(),
        }
    }

    pub fn get(&self, address: &MemoryAddress) -> Result<DataHolder, RuntimeError> {
        let item = match *address {
            MemoryAddress::Stack(ref item) => match *item {
                MemoryItem::Bool(index) => DataHolder::Bool(self.bools.get_stack(index)?),
                MemoryItem::Integer(index) => DataHolder::Integer(self.integers.get_stack(index)?),
                MemoryItem::Float(index) => DataHolder::Float(self.floats.get_stack(index)?),
                MemoryItem::String(index) => DataHolder::String(self.strings.get_stack(index)?),
                MemoryItem::Array(index) => DataHolder::Array(self.arrays.get_stack(index)?),
                MemoryItem::Function(index) => {
                    DataHolder::Function(self.functions.get_stack(index)?)
                }
            },
            MemoryAddress::Fixed(ref item) => self.get_fixed(item)?,
        };
        Ok(item)
    }

    pub fn pop(&mut self, address: &MemoryAddress) -> Result<DataHolder, RuntimeError> {
        let item = match *address {
            MemoryAddress::Stack(ref item) => match *item {
                MemoryItem::Bool(index) => DataHolder::Bool(self.bools.pop_stack(index)?),
                MemoryItem::Integer(index) => DataHolder::Integer(self.integers.pop_stack(index)?),
                MemoryItem::Float(index) => DataHolder::Float(self.floats.pop_stack(index)?),
                MemoryItem::String(index) => DataHolder::String(self.strings.pop_stack(index)?),
                MemoryItem::Array(index) => DataHolder::Array(self.arrays.pop_stack(index)?),
                MemoryItem::Function(index) => {
                    DataHolder::Function(self.functions.pop_stack(index)?)
                }
            },
            MemoryAddress::Fixed(ref item) => self.get_fixed(item)?,
        };
        Ok(item)
    }

    pub fn insert_stack(&mut self, data: DataHolder) -> MemoryAddress {
        let item = match data {};
        MemoryAddress::Stack(item)
    }

    fn get_fixed(&self, item: &MemoryItem) -> Result<DataHolder, RuntimeError> {
        let item = match *item {
            MemoryItem::Bool(index) => DataHolder::Bool(self.bools.get_fixed(index)?),
            MemoryItem::Integer(index) => DataHolder::Integer(self.integers.get_fixed(index)?),
            MemoryItem::Float(index) => DataHolder::Float(self.floats.get_fixed(index)?),
            MemoryItem::String(index) => DataHolder::String(self.strings.get_fixed(index)?),
            MemoryItem::Array(index) => DataHolder::Array(self.arrays.get_fixed(index)?),
            MemoryItem::Function(index) => DataHolder::Function(self.functions.get_fixed(index)?),
        };
        Ok(item)
    }
}

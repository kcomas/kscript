use super::command::Command;
use super::data::{DataHolder, RefDataHolder};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct MemoryItem<T: Clone> {
    value: T,
    total_refs: usize,
    constant: bool,
}

impl<T> MemoryItem<T>
where
    T: Clone,
{
    pub fn new(value: T, constant: bool) -> MemoryItem<T> {
        MemoryItem {
            value: value,
            total_refs: 1,
            constant: constant,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn update(&mut self, value: T) {
        if !self.constant {
            self.value = value;
        }
    }

    pub fn inc(&mut self) {
        if !self.constant {
            self.total_refs += 1;
        }
    }

    pub fn dec(&mut self) -> usize {
        if !self.constant {
            self.total_refs -= 1;
        }
        self.total_refs
    }
}

#[derive(Debug)]
pub struct MemoryContainer<T: Clone> {
    items: Vec<MemoryItem<T>>,
    free: VecDeque<usize>,
}

impl<T> MemoryContainer<T>
where
    T: Clone,
{
    pub fn new() -> MemoryContainer<T> {
        MemoryContainer {
            items: Vec::new(),
            free: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, value: T, constant: bool) -> usize {
        if let Some(pos) = self.free.pop_front() {
            self.items[pos] = MemoryItem::new(value, constant);
            return pos;
        }
        self.items.push(MemoryItem::new(value, constant));
        self.items.len() - 1
    }

    pub fn get(&self, index: usize) -> &T {
        self.items[index].get()
    }

    pub fn update(&mut self, index: usize, value: T) {
        self.items[index].update(value);
    }

    pub fn inc(&mut self, index: usize) {
        self.items[index].inc();
    }

    pub fn dec(&mut self, index: usize) {
        let count = self.items[index].dec();
        if count == 0 {
            self.free.push_back(index);
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemoryAddress {
    Bool(usize),
    Integer(usize),
    Float(usize),
    Function(usize),
}

impl MemoryAddress {
    pub fn get_address(&self) -> usize {
        match *self {
            MemoryAddress::Bool(index)
            | MemoryAddress::Integer(index)
            | MemoryAddress::Float(index)
            | MemoryAddress::Function(index) => index,
        }
    }

    pub fn is_bool(&self) -> bool {
        if let MemoryAddress::Bool(_) = *self {
            return true;
        }
        false
    }

    pub fn is_function(&self) -> bool {
        if let MemoryAddress::Function(_) = *self {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    commands: Vec<Command>,
    num_args: usize,
}

impl Function {
    pub fn new(commands: Vec<Command>, num_args: usize) -> Function {
        Function {
            commands: commands,
            num_args: num_args,
        }
    }

    pub fn get_command(&self, index: usize) -> Option<Command> {
        if let Some(cmd) = self.commands.get(index) {
            return Some(cmd.clone());
        }
        None
    }

    pub fn get_args(&self) -> usize {
        self.num_args
    }
}

#[derive(Debug)]
pub struct Memory {
    bools: MemoryContainer<bool>,
    integers: MemoryContainer<i64>,
    floats: MemoryContainer<f64>,
    functions: MemoryContainer<Function>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bools: MemoryContainer::new(),
            integers: MemoryContainer::new(),
            floats: MemoryContainer::new(),
            functions: MemoryContainer::new(),
        }
    }

    pub fn get(&self, place: &MemoryAddress) -> RefDataHolder {
        match *place {
            MemoryAddress::Bool(index) => RefDataHolder::Bool(self.bools.get(index)),
            MemoryAddress::Integer(index) => RefDataHolder::Integer(self.integers.get(index)),
            MemoryAddress::Float(index) => RefDataHolder::Float(self.floats.get(index)),
            MemoryAddress::Function(index) => RefDataHolder::Function(self.functions.get(index)),
        }
    }

    pub fn get_bool(&self, index: usize) -> &bool {
        self.bools.get(index)
    }

    pub fn get_function(&self, index: usize) -> &Function {
        self.functions.get(index)
    }

    pub fn insert(&mut self, value: DataHolder, constant: bool) -> MemoryAddress {
        match value {
            DataHolder::Bool(b) => MemoryAddress::Bool(self.bools.insert(b, constant)),
            DataHolder::Integer(int) => MemoryAddress::Integer(self.integers.insert(int, constant)),
            DataHolder::Float(float) => MemoryAddress::Float(self.floats.insert(float, constant)),
            DataHolder::Function(function) => {
                MemoryAddress::Function(self.functions.insert(function, constant))
            }
        }
    }

    pub fn update(&mut self, place: &MemoryAddress, value: DataHolder) {
        match *place {
            MemoryAddress::Bool(index) => {
                if let DataHolder::Bool(b) = value {
                    self.bools.update(index, b);
                }
            }
            MemoryAddress::Integer(index) => {
                if let DataHolder::Integer(int) = value {
                    self.integers.update(index, int);
                }
            }
            MemoryAddress::Float(index) => {
                if let DataHolder::Float(float) = value {
                    self.floats.update(index, float);
                }
            }
            MemoryAddress::Function(index) => {
                if let DataHolder::Function(function) = value {
                    self.functions.update(index, function);
                }
            }
        }
    }

    pub fn inc(&mut self, place: &MemoryAddress) {
        match *place {
            MemoryAddress::Bool(index) => self.bools.inc(index),
            MemoryAddress::Integer(index) => self.integers.inc(index),
            MemoryAddress::Float(index) => self.floats.inc(index),
            MemoryAddress::Function(index) => self.functions.inc(index),
        }
    }

    pub fn dec(&mut self, place: &MemoryAddress) {
        match *place {
            MemoryAddress::Bool(index) => self.bools.dec(index),
            MemoryAddress::Integer(index) => self.integers.dec(index),
            MemoryAddress::Float(index) => self.floats.dec(index),
            MemoryAddress::Function(index) => self.functions.dec(index),
        }
    }
}

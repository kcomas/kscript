use std::collections::VecDeque;
use super::command::Command;
use super::data::{DataHolder, RefDataHolder};
use super::error::RuntimeError;

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

    pub fn can_clear_constant(&mut self) -> bool {
        if self.constant {
            self.total_refs = 0;
            self.constant = false;
            return true;
        }
        false
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_ref_count(&self) -> usize {
        self.total_refs
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

    pub fn clear(&mut self) {
        self.total_refs = 0;
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

    pub fn get_ref_count(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.items.get(index) {
            return Ok(item.get_ref_count());
        }
        Err(RuntimeError::InvalidMemoryAccess)
    }

    pub fn insert(&mut self, value: T, constant: bool) -> usize {
        if let Some(pos) = self.free.pop_front() {
            self.items[pos] = MemoryItem::new(value, constant);
            return pos;
        }
        self.items.push(MemoryItem::new(value, constant));
        self.items.len() - 1
    }

    pub fn get(&self, index: usize) -> Result<&T, RuntimeError> {
        if let Some(item) = self.items.get(index) {
            return Ok(item.get());
        }
        Err(RuntimeError::InvalidMemoryAccess)
    }

    pub fn update(&mut self, index: usize, value: T) -> Result<(), RuntimeError> {
        if let Some(item) = self.items.get_mut(index) {
            item.update(value);
            return Ok(());
        }
        Err(RuntimeError::InvalidMemoryUpdate)
    }

    pub fn inc(&mut self, index: usize) -> Result<(), RuntimeError> {
        if let Some(item) = self.items.get_mut(index) {
            item.inc();
            return Ok(());
        }
        Err(RuntimeError::CannotIncreaseRefCount)
    }

    pub fn dec(&mut self, index: usize) -> Result<usize, RuntimeError> {
        if let Some(item) = self.items.get_mut(index) {
            let count = item.dec();
            if count == 0 {
                self.free.push_back(index);
            }
            return Ok(count);
        }
        Err(RuntimeError::CannotDecreaseRefCount)
    }

    pub fn clear(&mut self, index: usize) -> Result<(), RuntimeError> {
        if let Some(item) = self.items.get_mut(index) {
            item.clear();
            self.free.push_back(index);
            return Ok(());
        }
        Err(RuntimeError::CannotClearMemory)
    }

    pub fn clear_constant(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            if item.can_clear_constant() {
                self.free.push_back(index);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemoryAddress {
    Bool(usize),
    Integer(usize),
    Float(usize),
    String(usize),
    Array(usize),
    Function(usize),
}

impl MemoryAddress {
    pub fn get_address(&self) -> usize {
        match *self {
            MemoryAddress::Bool(index)
            | MemoryAddress::Integer(index)
            | MemoryAddress::Float(index)
            | MemoryAddress::String(index)
            | MemoryAddress::Array(index)
            | MemoryAddress::Function(index) => index,
        }
    }

    pub fn get_type_byte(&self) -> u8 {
        match *self {
            MemoryAddress::Bool(_) => 1,
            MemoryAddress::Integer(_) => 2,
            MemoryAddress::Float(_) => 3,
            MemoryAddress::String(_) => 4,
            MemoryAddress::Array(_) => 5,
            MemoryAddress::Function(_) => 6,
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

    pub fn get_memory_addresses(&self) -> Vec<MemoryAddress> {
        let mut addresses = Vec::new();
        for cmd in self.commands.iter() {
            if let Command::PushStack(ref address) = *cmd {
                addresses.push(address.clone());
            }
        }
        addresses
    }
}

#[derive(Debug)]
pub struct Memory {
    bools: MemoryContainer<bool>,
    integers: MemoryContainer<i64>,
    floats: MemoryContainer<f64>,
    strings: MemoryContainer<String>,
    arrays: MemoryContainer<Vec<MemoryAddress>>,
    functions: MemoryContainer<Function>,
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

    pub fn get(&self, place: &MemoryAddress) -> Result<RefDataHolder, RuntimeError> {
        let rst = match *place {
            MemoryAddress::Bool(index) => RefDataHolder::Bool(self.bools.get(index)?),
            MemoryAddress::Integer(index) => RefDataHolder::Integer(self.integers.get(index)?),
            MemoryAddress::Float(index) => RefDataHolder::Float(self.floats.get(index)?),
            MemoryAddress::String(index) => RefDataHolder::String(self.strings.get(index)?),
            MemoryAddress::Array(index) => RefDataHolder::Array(self.arrays.get(index)?),
            MemoryAddress::Function(index) => RefDataHolder::Function(self.functions.get(index)?),
        };
        Ok(rst)
    }

    pub fn get_bool(&self, index: usize) -> Result<&bool, RuntimeError> {
        self.bools.get(index)
    }

    pub fn get_integer(&self, index: usize) -> Result<&i64, RuntimeError> {
        self.integers.get(index)
    }

    pub fn get_function(&self, index: usize) -> Result<&Function, RuntimeError> {
        self.functions.get(index)
    }

    pub fn insert(&mut self, value: DataHolder, constant: bool) -> MemoryAddress {
        match value {
            DataHolder::Bool(b) => MemoryAddress::Bool(self.bools.insert(b, constant)),
            DataHolder::Integer(int) => MemoryAddress::Integer(self.integers.insert(int, constant)),
            DataHolder::Float(float) => MemoryAddress::Float(self.floats.insert(float, constant)),
            DataHolder::String(string) => {
                MemoryAddress::String(self.strings.insert(string, constant))
            }
            DataHolder::Array(array) => MemoryAddress::Array(self.arrays.insert(array, constant)),
            DataHolder::Function(function) => {
                MemoryAddress::Function(self.functions.insert(function, constant))
            }
        }
    }

    pub fn inc(&mut self, place: &MemoryAddress) -> Result<(), RuntimeError> {
        match *place {
            MemoryAddress::Bool(index) => self.bools.inc(index),
            MemoryAddress::Integer(index) => self.integers.inc(index),
            MemoryAddress::Float(index) => self.floats.inc(index),
            MemoryAddress::String(index) => self.strings.inc(index),
            MemoryAddress::Array(index) => self.arrays.inc(index),
            MemoryAddress::Function(index) => self.functions.inc(index),
        }
    }

    pub fn dec(&mut self, place: &MemoryAddress) -> Result<usize, RuntimeError> {
        match *place {
            MemoryAddress::Bool(index) => self.bools.dec(index),
            MemoryAddress::Integer(index) => self.integers.dec(index),
            MemoryAddress::Float(index) => self.floats.dec(index),
            MemoryAddress::String(index) => self.strings.dec(index),
            MemoryAddress::Array(index) => self.arrays.dec(index),
            MemoryAddress::Function(index) => {
                let count = self.functions.dec(index)?;
                if count == 0 {
                    self.clear_function(index)?;
                }
                Ok(count)
            }
        }
    }

    pub fn clear(&mut self, place: &MemoryAddress) -> Result<(), RuntimeError> {
        match *place {
            MemoryAddress::Bool(index) => self.bools.clear(index),
            MemoryAddress::Integer(index) => self.integers.clear(index),
            MemoryAddress::Float(index) => self.floats.clear(index),
            MemoryAddress::String(index) => self.strings.clear(index),
            MemoryAddress::Array(index) => self.arrays.clear(index),
            MemoryAddress::Function(index) => {
                self.clear_function(index);
                self.functions.clear(index)
            }
        }
    }

    pub fn clone_memory(&mut self, place: &MemoryAddress) -> Result<MemoryAddress, RuntimeError> {
        let c = match *place {
            MemoryAddress::Bool(index) => {
                let new_bool = self.bools.get(index)?.clone();
                MemoryAddress::Bool(self.bools.insert(new_bool, false))
            }
            MemoryAddress::Integer(index) => {
                let new_int = self.integers.get(index)?.clone();
                MemoryAddress::Integer(self.integers.insert(new_int, false))
            }
            MemoryAddress::Float(index) => {
                let new_float = self.floats.get(index)?.clone();
                MemoryAddress::Float(self.floats.insert(new_float, false))
            }
            MemoryAddress::String(index) => {
                let new_string = self.strings.get(index)?.clone();
                MemoryAddress::String(self.strings.insert(new_string, false))
            }
            MemoryAddress::Array(index) => {
                let new_array = self.clone_array(index)?;
                MemoryAddress::Array(self.arrays.insert(new_array, false))
            }
            MemoryAddress::Function(index) => {
                let new_function = self.functions.get(index)?.clone();
                MemoryAddress::Function(self.functions.insert(new_function, false))
            }
        };
        Ok(c)
    }

    pub fn update(
        &mut self,
        target: &MemoryAddress,
        source: &MemoryAddress,
    ) -> Result<(), RuntimeError> {
        let target_type_byte = target.get_type_byte();
        let source_type_byte = source.get_type_byte();

        if target_type_byte != source_type_byte {
            return Err(RuntimeError::UpdateTypeMismatch);
        }

        let target_index = target.get_address();
        let source_index = source.get_address();

        match target_type_byte {
            1 => {
                let value = self.bools.get(source_index)?.clone();
                self.bools.update(target_index, value)?;
            }
            2 => {
                let value = self.integers.get(source_index)?.clone();
                self.integers.update(target_index, value)?;
            }
            3 => {
                let value = self.floats.get(source_index)?.clone();
                self.floats.update(target_index, value)?;
            }
            4 => {
                let value = self.strings.get(source_index)?.clone();
                self.strings.update(target_index, value)?;
            }
            5 => {
                let value = self.clone_array(source_index)?;
                self.arrays.update(target_index, value)?;
            }
            6 => {
                let value = self.functions.get(source_index)?.clone();
                self.functions.update(target_index, value)?;
            }
            _ => return Err(RuntimeError::InvalidType),
        };
        Ok(())
    }

    pub fn clear_function(&mut self, index: usize) -> Result<(), RuntimeError> {
        let addresses = {
            let function = self.get_function(index)?;
            function.get_memory_addresses()
        };
        for address in addresses.iter() {
            self.clear_constant(address);
        }
        Ok(())
    }

    fn clone_array(&mut self, index: usize) -> Result<Vec<MemoryAddress>, RuntimeError> {
        let mut new_array = Vec::new();
        let mut to_clone = Vec::new();
        for item in self.arrays.get(index)?.iter() {
            to_clone.push(item.clone());
        }
        for c in to_clone.iter() {
            new_array.push(self.clone_memory(c)?);
        }
        Ok(new_array)
    }

    pub fn clear_constant(&mut self, place: &MemoryAddress) {
        match *place {
            MemoryAddress::Bool(index) => self.bools.clear_constant(index),
            MemoryAddress::Integer(index) => self.integers.clear_constant(index),
            MemoryAddress::Float(index) => self.floats.clear_constant(index),
            MemoryAddress::String(index) => self.strings.clear_constant(index),
            MemoryAddress::Array(index) => self.arrays.clear_constant(index),
            MemoryAddress::Function(index) => self.functions.clear_constant(index),
        }
    }
}

use std::collections::VecDeque;
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

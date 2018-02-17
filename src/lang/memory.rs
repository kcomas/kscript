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

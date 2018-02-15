use super::memory::MemoryAddress;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
    stack_function_index: usize,
}

use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Add,
    Halt(i32),
}

use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Halt(i32),
}

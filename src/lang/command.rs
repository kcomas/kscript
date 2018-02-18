use super::address::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Add,
    Sub,
    Halt(i32),
}

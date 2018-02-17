use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Add,
    Sub,
    Call,
    Return,
    Print,
    Halt(i32),
}

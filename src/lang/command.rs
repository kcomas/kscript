use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Equals,
    JumpIfFalse(usize),
    Add,
    Sub,
    Call,
    Return,
    LoadArg(usize),
    Print,
    Halt(i32),
}

use super::address::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Equals,
    JumpIfFalse(usize),
    Add,
    Sub,
    Call,
    LoadArg(usize),
    Return,
    Halt(i32),
}

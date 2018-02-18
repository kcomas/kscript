use super::address::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Add,
    Sub,
    Call,
    LoadArg(usize),
    Return,
    Halt(i32),
}

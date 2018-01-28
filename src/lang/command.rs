use super::memory::MemoryAddress;

#[derive(Debug, Clone)]
pub enum Command {
    PushStack(MemoryAddress),
    // Assign,
    Equals,
    Add,
    Sub,
    Call,
    LoadArgument(usize),
    Return,
    PrintDebug,
    Halt(i32),
}

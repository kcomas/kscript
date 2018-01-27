use super::memory::MemoryAddress;

#[derive(Debug, Clone)]
pub enum Command {
    PushStack(MemoryAddress),
    // Assign,
    Add,
    //  Sub,
    //  Call,
    //  Return,
    Halt(i32),
}

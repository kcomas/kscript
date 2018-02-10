use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    PushStack(MemoryAddress),
    Add,
    Sub,
    Halt(i32),
}

#[derive(Debug)]
pub enum RuntimeError {
    CannotLoadMemoryStackItem,
    CannotLoadMemoryFixedItem,
    CannotPopMemoryStack,
}
